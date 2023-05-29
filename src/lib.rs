use leptos::*;

#[component]
pub fn ForBug(cx: Scope) -> impl IntoView {
    // Lets have a signal holding a vector
    let (elements, _) = create_signal(cx, vec![1, 4, 2, 5, 3]);

    // Lets also have a signal storing a boolean to sort either in asc or desc order.
    let (reverse_sort, set_reverse_sort) = create_signal(cx, false);

    // Lets have a derived signal that sorts the array according to the sort order.
    let sorted_elements = move || {
        let mut elements = elements.get();
        elements.sort();
        if reverse_sort.get() {
            elements.reverse();
        }
        elements
    };

    view! { cx,
        <div>"This list will reverse fine"</div>
        <ul>
            {move || sorted_elements().into_iter().map(|element| {
                view! { cx, <li>{element}</li> }
            }).collect::<Vec<_>>()}
        </ul>

        <div>"This list will break when reversed"</div>
        <ul>
            <For
                each=sorted_elements
                key=|v| *v
                view=move |cx, element| {
                    view! { cx, <li>{element}</li> }
                }
            />
        </ul>
        <button on:click=move |_| set_reverse_sort.update(|v| *v = !*v)>"Reverse sort"</button>
    }
}
