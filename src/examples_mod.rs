// examples_mod.rs

use crate::*;

// init texts, load from storage or default
pub fn init_texts() {
    //load the texts to local storage
    let regex_text = load_string_from_local_storage("regex_text", r#"<title>(.+?)</title>"#);
    set_text_area_element_value_string_by_id("regex_text", &regex_text);
    let substitution = load_string_from_local_storage("substitution", "<name>$1</name>");
    set_text_area_element_value_string_by_id("substitution", &substitution);
    let test_string = load_string_from_local_storage(
        "test_string",
        r#"<catalog>
    <movie>
        <title>The Terminator</title>
        <year>1984</year>
    </movie>
    <movie>
        <title>Terminator 2: Judgment Day</title>
        <year>1991</year>
    </movie>
    <movie>
        <title>Terminator 3: Rise of the Machines</title>
        <year>2003</year>
    </movie>
    <movie>
        <title>Terminator Salvation</title>
        <year>2009</year>
    </movie>
    <movie>
        <title>Terminator Genisys</title>
        <year>2015</year>
    </movie>
    <movie>
        <title>Terminator: Dark Fate</title>
        <year>2019</year>
    </movie>
</catalog>
  "#,
    );
    let test_string = HtmlEncoded::from_str(&test_string);
    set_element_inner_html_by_id("test_string", &test_string);

    run_regex();
}

// example email
pub fn example_email() {
    let regex_text = r#"^[a-zA-Z0-9.!#$%&’*+/=?^_`{|}~-]+@[a-zA-Z0-9-]+\.([a-zA-Z0-9-]+)*"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    let substitution = "The email domain is: $1";
    set_text_area_element_value_string_by_id("substitution", substitution);
    let test_string = HtmlEncoded::from_str("John.Connor.42@sky.net");
    set_element_inner_html_by_id("test_string", &test_string);
    set_element_inner_html_by_id("test_string", &test_string);
    run_regex();
}

// example model_base
pub fn example_model_base() {
    let regex_text = r#"T-"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    let substitution = "Robot($1)";
    set_text_area_element_value_string_by_id("substitution", substitution);
    let test_string = HtmlEncoded::from_str(
        r#"T-1000 (Robert Patrick) Terminator known as T-101 T-800 that managed to kill John Connor explicitly named T-600s and T-1000. it jams its remaining hydrogen fuel cell into the T-X's mouth from a T-1000 sent to kill her who has been transformed into a T-3000 improvement over the earlier T-600 units also refers to the character as T-850 used the T-800 and T-850 nomenclature memory of a T-888 model, tearing a malfunctioning T-600 in half"#,
    );
    set_element_inner_html_by_id("test_string", &test_string);
}

// example model1
pub fn example_model_1() {
    example_model_base();
    run_regex();
}

// example model2
pub fn example_model_2() {
    example_model_base();
    let regex_text = r#"T-\d+"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    run_regex();
}

// example model3
pub fn example_model_3() {
    example_model_base();
    let regex_text = r#"T-(X|\d+)"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    run_regex();
}

// example xml_1
pub fn example_xml_1() {
    example_xml_1_base();
    run_regex();
}

// example xml_2
pub fn example_xml_2() {
    example_xml_1_base();
    let regex_text = r#"<year>(1[789]\d\d)</year>"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    run_regex();
}

// example xml_3
pub fn example_xml_3() {
    example_xml_1_base();
    let regex_text = r#"<year>(2\d{3})</year>"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    run_regex();
}

// example xml_1 base
pub fn example_xml_1_base() {
    let regex_text = r#"<title>(.+?)</title>"#;
    set_text_area_element_value_string_by_id("regex_text", regex_text);
    let substitution = "<name>$1</name>";
    set_text_area_element_value_string_by_id("substitution", substitution);
    let test_string = HtmlEncoded::from_str(
        r#"<catalog>
    <movie>
        <title>The Terminator</title>
        <year>1984</year>
    </movie>
    <movie>
        <title>Terminator 2: Judgment Day</title>
        <year>1991</year>
    </movie>
    <movie>
        <title>Terminator 3: Rise of the Machines</title>
        <year>2003</year>
    </movie>
    <movie>
        <title>Terminator Salvation</title>
        <year>2009</year>
    </movie>
    <movie>
        <title>Terminator Genisys</title>
        <year>2015</year>
    </movie>
    <movie>
        <title>Terminator: Dark Fate</title>
        <year>2019</year>
    </movie>
</catalog>
  "#,
    );
    set_element_inner_html_by_id("test_string", &test_string);
}
