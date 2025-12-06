//! Slide XML generation for different layouts

use super::slide_content::{SlideContent, SlideLayout};
use super::package_xml::escape_xml;
use super::shapes_xml::generate_shape_xml;
use super::slide::formatting::{generate_rich_text_runs, generate_text_props};

/// Create simple slide XML
pub fn create_slide_xml(slide_num: usize, title: &str) -> String {
    let slide_title = if slide_num == 1 {
        title.to_string()
    } else {
        format!("Slide {slide_num}")
    };
    
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="0" cy="0"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="0" cy="0"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title 1"/>
<p:cNvSpPr>
<a:spLocks noGrp="1"/>
</p:cNvSpPr>
<p:nvPr>
<p:ph type="ctrTitle"/>
</p:nvPr>
</p:nvSpPr>
<p:spPr/>
<p:txBody>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" smtClean="0"/>
<a:t>{slide_title}</a:t>
</a:r>
<a:endParaRPr lang="en-US"/>
</a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#
    )
}

/// Create slide XML with content based on layout
pub fn create_slide_xml_with_content(_slide_num: usize, content: &SlideContent) -> String {
    match content.layout {
        SlideLayout::Blank => create_blank_slide(),
        SlideLayout::TitleOnly => create_title_only_slide(content),
        SlideLayout::CenteredTitle => create_centered_title_slide(content),
        SlideLayout::TitleAndBigContent => create_title_and_big_content_slide(content),
        SlideLayout::TwoColumn => create_two_column_slide(content),
        SlideLayout::TitleAndContent => create_title_and_content_slide(content),
    }
}

fn create_blank_slide() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:bg>
<p:bgRef idx="1001">
<a:schemeClr val="bg1"/>
</p:bgRef>
</p:bg>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="9144000" cy="6858000"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="9144000" cy="6858000"/>
</a:xfrm>
</p:grpSpPr>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#.to_string()
}

fn create_title_only_slide(content: &SlideContent) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:bg>
<p:bgRef idx="1001">
<a:schemeClr val="bg1"/>
</p:bgRef>
</p:bg>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="9144000" cy="6858000"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="9144000" cy="6858000"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="274638"/>
<a:ext cx="8230200" cy="1143000"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:pPr algn="l"/>
<a:r>
{title_props}
<a:t>{title_text}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#
    )
}

fn create_centered_title_slide(content: &SlideContent) -> String {
    let title_size = content.title_size.unwrap_or(54) * 100;
    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:bg>
<p:bgRef idx="1001">
<a:schemeClr val="bg1"/>
</p:bgRef>
</p:bg>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="9144000" cy="6858000"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="9144000" cy="6858000"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="2743200"/>
<a:ext cx="8230200" cy="1371600"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:pPr algn="ctr"/>
<a:r>
{title_props}
<a:t>{title_text}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#
    )
}

fn create_title_and_big_content_slide(content: &SlideContent) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let content_size = content.content_size.unwrap_or(28) * 100;

    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    let mut xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:bg>
<p:bgRef idx="1001">
<a:schemeClr val="bg1"/>
</p:bgRef>
</p:bg>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="9144000" cy="6858000"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="9144000" cy="6858000"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="274638"/>
<a:ext cx="8230200" cy="914400"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:pPr algn="l"/>
<a:r>
{title_props}
<a:t>{title_text}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>"#
    );

    if !content.content.is_empty() {
        xml.push_str(
            r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Content"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="1189200"/>
<a:ext cx="8230200" cy="5668800"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#
        );

        for bullet in content.content.iter() {
            let rich_text = generate_rich_text_runs(
                bullet,
                content_size,
                content.content_bold,
                content.content_italic,
                content.content_color.as_deref(),
            );
            xml.push_str(&format!(
                r#"
<a:p>
<a:pPr lvl="0"/>
{rich_text}
</a:p>"#
            ));
        }

        xml.push_str(
            r#"
</p:txBody>
</p:sp>"#
        );
    }

    xml.push_str(
        r#"
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#
    );

    xml
}

fn create_two_column_slide(content: &SlideContent) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let content_size = content.content_size.unwrap_or(24) * 100;

    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    let mut xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:bg>
<p:bgRef idx="1001">
<a:schemeClr val="bg1"/>
</p:bgRef>
</p:bg>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="9144000" cy="6858000"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="9144000" cy="6858000"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="274638"/>
<a:ext cx="8230200" cy="914400"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:pPr algn="l"/>
<a:r>
{title_props}
<a:t>{title_text}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>"#
    );

    if !content.content.is_empty() {
        let mid = content.content.len().div_ceil(2);
        let left_content = &content.content[..mid];
        let right_content = &content.content[mid..];

        // Left column
        xml.push_str(
            r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Left Content"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="1189200"/>
<a:ext cx="4115100" cy="5668800"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#
        );

        for bullet in left_content.iter() {
            let rich_text = generate_rich_text_runs(
                bullet,
                content_size,
                content.content_bold,
                content.content_italic,
                content.content_color.as_deref(),
            );
            xml.push_str(&format!(
                r#"
<a:p>
<a:pPr lvl="0"/>
{rich_text}
</a:p>"#
            ));
        }

        xml.push_str(
            r#"
</p:txBody>
</p:sp>"#
        );

        // Right column
        if !right_content.is_empty() {
            xml.push_str(
                r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="4" name="Right Content"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="4572300" y="1189200"/>
<a:ext cx="4115100" cy="5668800"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#
            );

            for bullet in right_content.iter() {
                let rich_text = generate_rich_text_runs(
                    bullet,
                    content_size,
                    content.content_bold,
                    content.content_italic,
                    content.content_color.as_deref(),
                );
                xml.push_str(&format!(
                    r#"
<a:p>
<a:pPr lvl="0"/>
{rich_text}
</a:p>"#
                ));
            }

            xml.push_str(
                r#"
</p:txBody>
</p:sp>"#
            );
        }
    }

    xml.push_str(
        r#"
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#
    );

    xml
}

fn create_title_and_content_slide(content: &SlideContent) -> String {
    let title_size = content.title_size.unwrap_or(44) * 100;
    let content_size = content.content_size.unwrap_or(28) * 100;

    let title_props = generate_text_props(
        title_size,
        content.title_bold,
        content.title_italic,
        content.title_underline,
        content.title_color.as_deref(),
    );
    let title_text = escape_xml(&content.title);

    let mut xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
<p:cSld>
<p:bg>
<p:bgRef idx="1001">
<a:schemeClr val="bg1"/>
</p:bgRef>
</p:bg>
<p:spTree>
<p:nvGrpSpPr>
<p:cNvPr id="1" name=""/>
<p:cNvGrpSpPr/>
<p:nvPr/>
</p:nvGrpSpPr>
<p:grpSpPr>
<a:xfrm>
<a:off x="0" y="0"/>
<a:ext cx="9144000" cy="6858000"/>
<a:chOff x="0" y="0"/>
<a:chExt cx="9144000" cy="6858000"/>
</a:xfrm>
</p:grpSpPr>
<p:sp>
<p:nvSpPr>
<p:cNvPr id="2" name="Title"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="274638"/>
<a:ext cx="8230200" cy="1143000"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:pPr algn="l"/>
<a:r>
{title_props}
<a:t>{title_text}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>"#
    );

    // Render table if present
    if let Some(ref table) = content.table {
        xml.push('\n');
        xml.push_str(&super::tables_xml::generate_table_xml(table, 3));
    } else if !content.content.is_empty() {
        // Render bullets if no table
        xml.push_str(
            r#"
<p:sp>
<p:nvSpPr>
<p:cNvPr id="3" name="Content"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="457200" y="1600200"/>
<a:ext cx="8230200" cy="4572000"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:noFill/>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0"/>
<a:lstStyle/>"#
        );

        for bullet in content.content.iter() {
            let rich_text = generate_rich_text_runs(
                bullet,
                content_size,
                content.content_bold,
                content.content_italic,
                content.content_color.as_deref(),
            );
            xml.push_str(&format!(
                r#"
<a:p>
<a:pPr lvl="0"/>
{rich_text}
</a:p>"#
            ));
        }

        xml.push_str(
            r#"
</p:txBody>
</p:sp>"#
        );
    }

    // Render shapes if present
    // Start shape IDs after title (2) and content (3)
    for (i, shape) in content.shapes.iter().enumerate() {
        xml.push('\n');
        xml.push_str(&generate_shape_xml(shape, (i + 10) as u32));
    }

    // Note: Images require actual image data embedded in ppt/media/ and 
    // corresponding relationships. For now, we add a placeholder shape showing
    // where the image would be placed.
    let image_start_id = 20 + content.shapes.len();
    for (i, image) in content.images.iter().enumerate() {
        xml.push('\n');
        // Create a placeholder rectangle showing image location
        let id = image_start_id + i;
        let filename = &image.filename;
        let x = image.x;
        let y = image.y;
        let width = image.width;
        let height = image.height;
        xml.push_str(&format!(
            r#"<p:sp>
<p:nvSpPr>
<p:cNvPr id="{id}" name="Image Placeholder: {filename}"/>
<p:cNvSpPr/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="{x}" y="{y}"/>
<a:ext cx="{width}" cy="{height}"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:solidFill><a:srgbClr val="E0E0E0"/></a:solidFill>
<a:ln w="12700"><a:solidFill><a:srgbClr val="808080"/></a:solidFill></a:ln>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="ctr"/>
<a:lstStyle/>
<a:p>
<a:pPr algn="ctr"/>
<a:r>
<a:rPr lang="en-US" sz="1400"/>
<a:t>📷 {filename}</a:t>
</a:r>
</a:p>
</p:txBody>
</p:sp>"#
        ));
    }

    // Render code blocks with syntax highlighting
    let code_start_id = 30 + content.shapes.len() + content.images.len();
    for (i, code_block) in content.code_blocks.iter().enumerate() {
        xml.push('\n');
        let id = code_start_id + i;
        let highlighted_xml = crate::cli::syntax::generate_highlighted_code_xml(&code_block.code, &code_block.language);
        let x = code_block.x;
        let y = code_block.y;
        let width = code_block.width;
        let height = code_block.height;
        xml.push_str(&format!(
            r#"<p:sp>
<p:nvSpPr>
<p:cNvPr id="{id}" name="Code Block"/>
<p:cNvSpPr txBox="1"/>
<p:nvPr/>
</p:nvSpPr>
<p:spPr>
<a:xfrm>
<a:off x="{x}" y="{y}"/>
<a:ext cx="{width}" cy="{height}"/>
</a:xfrm>
<a:prstGeom prst="rect"><a:avLst/></a:prstGeom>
<a:solidFill><a:srgbClr val="002B36"/></a:solidFill>
<a:ln w="12700"><a:solidFill><a:srgbClr val="073642"/></a:solidFill></a:ln>
</p:spPr>
<p:txBody>
<a:bodyPr wrap="square" rtlCol="0" anchor="t" lIns="91440" tIns="45720" rIns="91440" bIns="45720"/>
<a:lstStyle/>
{highlighted_xml}</p:txBody>
</p:sp>"#
        ));
    }

    // Render connectors
    let connector_start_id = 50 + content.shapes.len() + content.images.len() + content.code_blocks.len();
    for (i, connector) in content.connectors.iter().enumerate() {
        xml.push('\n');
        let id = connector_start_id + i;
        xml.push_str(&super::connectors::generate_connector_xml(connector, id));
    }

    xml.push_str(
        r#"
</p:spTree>
</p:cSld>
<p:clrMapOvr>
<a:masterClrMapping/>
</p:clrMapOvr>
</p:sld>"#
    );

    xml
}

/// Create slide relationships XML
pub fn create_slide_rels_xml() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/slideLayout" Target="../slideLayouts/slideLayout1.xml"/>
</Relationships>"#.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::slide::formatting::parse_inline_formatting;

    #[test]
    fn test_parse_inline_formatting_plain() {
        let segments = parse_inline_formatting("Hello world");
        assert_eq!(segments.len(), 1);
        assert_eq!(segments[0].text, "Hello world");
        assert!(!segments[0].bold);
        assert!(!segments[0].italic);
    }

    #[test]
    fn test_parse_inline_formatting_bold() {
        let segments = parse_inline_formatting("This is **bold** text");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[0].text, "This is ");
        assert!(!segments[0].bold);
        assert_eq!(segments[1].text, "bold");
        assert!(segments[1].bold);
        assert_eq!(segments[2].text, " text");
        assert!(!segments[2].bold);
    }

    #[test]
    fn test_parse_inline_formatting_italic() {
        let segments = parse_inline_formatting("This is *italic* text");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[1].text, "italic");
        assert!(segments[1].italic);
    }

    #[test]
    fn test_parse_inline_formatting_code() {
        let segments = parse_inline_formatting("Use `code` here");
        assert_eq!(segments.len(), 3);
        assert_eq!(segments[1].text, "code");
        assert!(segments[1].code);
    }

    #[test]
    fn test_parse_inline_formatting_mixed() {
        let segments = parse_inline_formatting("**bold** and *italic*");
        assert!(segments.iter().any(|s| s.bold && s.text == "bold"));
        assert!(segments.iter().any(|s| s.italic && s.text == "italic"));
    }

    #[test]
    fn test_generate_rich_text_runs_plain() {
        let xml = generate_rich_text_runs("Hello", 2800, false, false, None);
        assert!(xml.contains("<a:t>Hello</a:t>"));
        assert!(xml.contains(r#"b="0""#));
    }

    #[test]
    fn test_generate_rich_text_runs_bold() {
        let xml = generate_rich_text_runs("**bold**", 2800, false, false, None);
        assert!(xml.contains(r#"b="1""#));
        assert!(xml.contains("<a:t>bold</a:t>"));
    }

    #[test]
    fn test_generate_rich_text_runs_code() {
        let xml = generate_rich_text_runs("`code`", 2800, false, false, None);
        assert!(xml.contains(r#"typeface="Consolas""#));
        assert!(xml.contains("<a:t>code</a:t>"));
    }
}
