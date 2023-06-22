const r = "30";
const r_emph = "33";
const fill = "#ffffff"
const fill_hidden = "#949494"
const fill_emph = "#ffdbf8"
const fill_selected = "#c1ffb8"

let hidden = []

const hide_all = () => {
    let lang = get_lang(document.getElementById("lang-select").value)
    hidden = Array.from(lang.states)
    reload_lang(document.getElementById("lang-select"))
}

const show_all = () => {
    hidden = []
    reload_lang(document.getElementById("lang-select"))
}

const emph = (e) => {
    let coloured_elem = document.getElementById(`${e.id}c`)
    coloured_elem.setAttribute("r", r_emph)
    coloured_elem.setAttribute("fill", fill_emph)
}

const de_emph = (e) => {
    let coloured_elem = document.getElementById(`${e.id}c`)
    if (coloured_elem.getAttribute("fill") === fill_selected) { return }
    coloured_elem.setAttribute("r", r)
    coloured_elem.setAttribute("fill", hidden.includes(e.id) ? fill_hidden : fill)
}

const reload_lang = (e) => {
    let svg = document.getElementById("svg")
    let lang = get_lang(e.value)
    let states = lang.states;
    let accepting = lang.accepting;
    let transitions = lang.transitions;
    let positions = lang.positions;
    let i = 0
    let out = `<defs>
        <marker id="arrow"
          viewBox="0 0 10 10" refX="10" refY="5"
          markerWidth="12" markerHeight="12"
          orient="auto-start-reverse">
          <path d="M 0 0 L 10 5 L 0 10 z" />
        </marker>
      </defs>`
    for (const transition in transitions) {
        let [q0, q1] = transition.split(",")
        if (hidden.includes(q0) || hidden.includes(q1)) { continue }
        let p0 = positions[parseInt(q0.slice(1))]
        let p1 = positions[parseInt(q1.slice(1))]
        let pos_diff = [p1[0] - p0[0], p1[1] - p0[1]]
        let center_distance = Math.sqrt(pos_diff[0] * pos_diff[0] + pos_diff[1] * pos_diff[1])
        let arrow_offset = [pos_diff[0] * r / center_distance, pos_diff[1] * r / center_distance]
        let label = transitions[transition][0] ? `${transitions[transition][0]} (${transitions[transition][1]})` : transitions[transition][1]
        out += `<line x1="${p0[0] + arrow_offset[0]}" y1="${p0[1] + arrow_offset[1]}" x2="${p1[0] - arrow_offset[0]}" y2="${p1[1] - arrow_offset[1]}"
                  stroke="black" marker-end="url(#arrow)" id="t${q0}_${q1}" />
                <text text-anchor="middle" dy="-5" dx="${(center_distance - 2 * r) / 2}" font-size="15" font-family="Verdana" font-weight="normal" id="${transition}" onclick="alter_label(this)">
                  <textPath href="#t${q0}_${q1}" side="${p0[0] < p1[0] ? 'left' : 'right'}">${label}</textPath>
                </text>`
    }
    let width = Math.max(...positions.map((t) => t[0])) + 500
    let height = Math.max(...positions.map((t) => t[1])) + 500
    while (i < states.length) {
        let colour = hidden.includes(`q${i}`) ? fill_hidden : fill
        let pos = positions[i]
        out += `<circle cx="${pos[0]}" cy="${pos[1]}" r="${r}" stroke="black" stroke-width="${accepting[i] ? 4 : 1}" fill="${colour}" id="q${i}c"/>
         <text text-anchor="middle" fill="#000000" font-size="15" font-family="Verdana" x="${pos[0]}" y="${pos[1] + 6}">${states[i]}</text>
         <circle cx="${pos[0]}" cy="${pos[1]}" r="${r}" stroke="#0000" stroke-width="${accepting[i] ? 4 : 1}" fill="#0000"
           id="q${i}" onmouseover="emph(this)" onmouseout="de_emph(this)" onclick="select(event, this)"
         />`
        i++
    }
    svg.innerHTML = out
    svg.setAttribute("width", `${width + 2 * r}`)
    svg.setAttribute("height", `${height + 2 * r}`)
}
