use crate::use_list::UseList;
use dioxus::prelude::*;

#[derive(Props)]
pub struct ListProps<'a, F, G> {
    /// Length of the list.
    pub len: usize,

    /// Size of the container.
    pub size: f64,

    /// Size of each item.
    pub item_size: f64,

    /// Function to create a new item.
    pub make_item: F,

    /// Function to create a new value.
    pub make_value: G,

    /// Event handler for scroll events.
    pub onscroll: Option<EventHandler<'a>>,
}

/// Virtualized list component.
#[allow(non_snake_case)]
pub fn List<'a, T: 'static, F, G>(cx: Scope<'a, ListProps<'a, F, G>>) -> Element<'a>
where
    F: Fn(&T) -> Element<'a>,
    G: Fn(usize) -> T + Clone + 'static,
{
    let list = UseList::builder()
        .len(cx.props.len)
        .size(cx.props.size)
        .item_size(cx.props.item_size)
        .use_list(cx, cx.props.make_value.clone());

    let top_row = (*list.scroll.read() as f64 / *list.item_size.read()).floor() as usize;
    let values_ref = list.values.read();
    let rows = values_ref.iter().enumerate().map(|(idx, value)| {
        render!(
            div {
                position: "absolute",
                top: "{(top_row + idx) as f64 * *list.item_size.read()}px",
                left: 0,
                width: "100%",
                height: "{list.item_size.read()}px",
                overflow: "hidden",
                (cx.props.make_item)( value)
            }
        )
    });

    let size = *list.size.read();
    render!(
        div {
            height: "{size}px",
            overflow: "scroll",
            onmounted: move |event| list.mounted.onmounted(event),
            onscroll: move |_| {
                if let Some(mounted) = &*list.mounted.signal.read() {
                    let elem: &web_sys::Element = mounted
                        .get_raw_element()
                        .unwrap()
                        .downcast_ref()
                        .unwrap();
                    list.scroll.set(elem.scroll_top());
                }

                if let Some(handler)  = &cx.props.onscroll {
                    handler.call(())
                }
            },
            div {
                position: "relative",
                height: "{list.item_size * cx.props.len as f64}px",
                overflow: "hidden",
                rows
            }
        }
    )
}
