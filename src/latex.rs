pub fn to_latex(text: String) -> String {
    let mut latex = "$".to_string();
    latex.push_str(&text);
    latex.push_str("$");
    latex = latex.replace("A","\\forall ");
    latex = latex.replace("E","\\exists ");
    latex = latex.replace("*","\\cdot ");
    latex = latex.replace(">","\\Rightarrow ");
    latex = latex.replace("&","\\wedge ");
    latex = latex.replace("|","\\vee ");
    latex
}