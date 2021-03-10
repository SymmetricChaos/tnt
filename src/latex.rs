pub fn to_latex(text: String, dent: usize) -> String {
    let mut latex = "".to_string();
    latex.push_str(&format!("\\hspace*{{{}em}}",2*dent));

    let mut text = text;
    text = text.replace("A","\\forall ");
    text = text.replace("E","\\exists ");
    text = text.replace("*","\\cdot ");
    text = text.replace(">","\\rightarrow ");
    text = text.replace("&","\\wedge ");
    text = text.replace("|","\\vee ");
    text = text.replace("[","\\langle");
    text = text.replace("]","\\rangle");

    latex.push_str(&text);
    latex.push_str("\\\\");

    latex
}