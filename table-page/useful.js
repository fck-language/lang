const download = () => {
    let element = document.createElement('a');
    let svg = document.getElementById("svg")
    let text = `<svg width="${svg.width}" height="${svg.height}" xmlns="http://www.w3.org/2000/svg">${svg.innerHTML}</svg>`
    element.setAttribute("href", "data:text/plain;charset=utf-8," + encodeURIComponent(text));
    element.setAttribute("download", "graph.svg");
    element.style.display = "none";
    document.body.appendChild(element);
    element.click();
    document.body.removeChild(element);
}

const download_lang = () => {
    let selector = document.getElementById("lang-select").value
    let lang = get_lang(selector)
    selector = selector.match(/[a-zA-Z]* \(([a-z]*)\)/)[1]
    let element = document.createElement('a');
    let text = Object.entries(lang).map(([k, v]) => `${k}: ${v}`).join(', ')
    element.setAttribute("href", "data:text/plain;charset=utf-8," + encodeURIComponent(`{${text}}`));
    element.setAttribute("download", `${selector}.js`);
    element.style.display = "none";
    document.body.appendChild(element);
    element.click();
    document.body.removeChild(element);
}

const svg_x = document.getElementById("svg").getBoundingClientRect().x
const svg_y = document.getElementById("svg").getBoundingClientRect().y

let selected_elem = null
let selected_elem_type = "state"

const select = (ev, e) => {
    if (ev.shiftKey) {
        let index = hidden.indexOf(e.id)
        index === -1 ? hidden.push(e.id) : delete hidden[index]
        reload_lang(document.getElementById("lang-select"))
    } else {
        if (selected_elem === e.id) { selected_elem = null; return }
        selected_elem = e.id
        selected_elem_type = "state"
        document.getElementById(`${e.id}c`).setAttribute("fill", fill_selected)
    }
}

const alter_label = (e) => {
    selected_elem = e.id
    e.setAttribute("font-size", "20")
    e.setAttribute("font-weight", "bold")
    selected_elem_type = "transition"
}

const rename_transition = (event, elem) => {
    if (event.key !== "Enter") { return }
    if (selected_elem_type !== "transition" || selected_elem === null) { return }
    let lang_str = document.getElementById("lang-select").value
    let lang = get_lang(lang_str)
    lang.transitions[selected_elem][0] = elem.value
    elem.value = ""
    elem.blur()
    document.getElementById(selected_elem).setAttribute("font-size", "15")
    document.getElementById(selected_elem).setAttribute("font-weight", "normal")
    selected_elem = null
    reload_lang({value: lang_str})
}

document.addEventListener("keydown", (k) => {
    if (selected_elem !== null && k.key === "Escape") {
        if (selected_elem_type === "transition") {
            document.getElementById(selected_elem).setAttribute("font-size", "15")
            document.getElementById(selected_elem).setAttribute("font-weight", "normal")
        } else {
            // state
            document.getElementById(`${selected_elem}c`).setAttribute("fill", fill)
            document.getElementById(`${selected_elem}c`).setAttribute("r", r)
        }
        selected_elem = null
        k.preventDefault()
    }
}, false)

document.getElementById("svg").addEventListener("click", (ev) => {
    if (selected_elem === null) { return }
    if (selected_elem_type === "transition") {
        document.getElementById(selected_elem).setAttribute("font-size", "15")
        document.getElementById(selected_elem).setAttribute("font-weight", "normal")
        selected_elem = null
        return
    }
    let lang_str = document.getElementById("lang-select").value
    let lang = get_lang(lang_str)
    let el = document.getElementById(selected_elem)
    let index = lang.states.indexOf(el.id)
    lang.positions[index] = [ev.pageX - svg_x, ev.pageY - svg_y]
    reload_lang({value: lang_str})
    document.getElementById(`${selected_elem}c`).setAttribute("fill", fill)
    selected_elem = null
}, true)
