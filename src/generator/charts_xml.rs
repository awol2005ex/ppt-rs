//! Chart XML generation for PPTX presentations

use crate::generator::charts::{Chart, ChartType};

/// Generate chart XML for a slide
pub fn generate_chart_xml(chart: &Chart, shape_id: usize) -> String {
    match chart.chart_type {
        ChartType::Bar => generate_bar_chart_xml(chart, shape_id),
        ChartType::Line => generate_line_chart_xml(chart, shape_id),
        ChartType::Pie => generate_pie_chart_xml(chart, shape_id),
    }
}

/// Generate bar chart XML
fn generate_bar_chart_xml(chart: &Chart, shape_id: usize) -> String {
    let mut xml = format!(
        r#"<p:graphicFrame>
<p:nvGraphicFramePr>
<p:cNvPr id="{}" name="Chart {}"/>
<p:cNvGraphicFramePr/>
<p:nvPr/>
</p:nvGraphicFramePr>
<p:xfrm>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</p:xfrm>
<a:graphic>
<a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<c:nvChartSpPr>
<c:cNvPr id="1" name="Chart"/>
<c:cNvChartSpPr/>
<c:nvPr/>
</c:nvChartSpPr>
<c:chartSpace>
<c:chart>
<c:title>
<c:tx>
<c:rich>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" sz="1800"/>
<a:t>{}</a:t>
</a:r>
</a:p>
</c:rich>
</c:tx>
</c:title>
<c:plotArea>
<c:layout/>
<c:barChart>
<c:barDir val="bar"/>
<c:grouping val="clustered"/>"#,
        shape_id,
        shape_id,
        chart.x,
        chart.y,
        chart.width,
        chart.height,
        escape_xml(&chart.title)
    );

    // Add series
    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:ser>
<c:idx val="{}"/>
<c:order val="{}"/>
<c:title>
<c:tx>
<c:rich>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" sz="1000"/>
<a:t>{}</a:t>
</a:r>
</a:p>
</c:rich>
</c:tx>
</c:title>
<c:dLbls>
<c:showVal val="0"/>
</c:dLbls>
<c:val>
<c:numRef>
<c:f>Sheet1!$B${}:$B${}</c:f>
<c:numCache>
<c:formatCode>General</c:formatCode>"#,
            idx,
            idx,
            escape_xml(&series.name),
            2 + idx,
            2 + idx + chart.series.len()
        ));

        for value in &series.values {
            xml.push_str(&format!(
                r#"
<c:pt idx="0">
<c:v>{}</c:v>
</c:pt>"#,
                value
            ));
        }

        xml.push_str(
            r#"
</c:numCache>
</c:numRef>
</c:val>
</c:ser>"#
        );
    }

    // Add category axis
    xml.push_str(
        r#"
<c:catAx>
<c:axId val="1"/>
<c:scaling>
<c:orientation val="minMax"/>
</c:scaling>
<c:delete val="0"/>
<c:axPos val="l"/>
<c:majorGridlines/>
<c:numFmt formatCode="General" sourceLinked="1"/>
<c:tickLblPos val="low"/>
<c:crossAx val="2"/>
<c:crosses val="autoZero"/>
<c:strRef>
<c:f>Sheet1!$A$2:$A$"#
    );

    xml.push_str(&format!("{}", 1 + chart.category_count()));
    xml.push_str(
        r#"</c:f>
<c:strCache>
<c:ptCount val=""#
    );

    xml.push_str(&format!("{}", chart.category_count()));
    xml.push_str("\">");

    for (idx, cat) in chart.categories.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
            idx, escape_xml(cat)
        ));
    }

    xml.push_str(
        r#"
</c:strCache>
</c:strRef>
</c:catAx>
<c:valAx>
<c:axId val="2"/>
<c:scaling>
<c:orientation val="minMax"/>
</c:scaling>
<c:delete val="0"/>
<c:axPos val="b"/>
<c:majorGridlines/>
<c:numFmt formatCode="General" sourceLinked="1"/>
<c:tickLblPos val="low"/>
<c:crossAx val="1"/>
<c:crosses val="autoZero"/>
</c:valAx>
</c:barChart>
</c:plotArea>
<c:legend>
<c:legendPos val="r"/>
<c:overlay val="0"/>
</c:legend>
<c:plotVisOnly val="1"/>
</c:chart>
</c:chartSpace>
</a:graphicData>
</a:graphic>
</p:graphicFrame>"#
    );

    xml
}

/// Generate line chart XML
fn generate_line_chart_xml(chart: &Chart, shape_id: usize) -> String {
    let mut xml = format!(
        r#"<p:graphicFrame>
<p:nvGraphicFramePr>
<p:cNvPr id="{}" name="Chart {}"/>
<p:cNvGraphicFramePr/>
<p:nvPr/>
</p:nvGraphicFramePr>
<p:xfrm>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</p:xfrm>
<a:graphic>
<a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<c:nvChartSpPr>
<c:cNvPr id="1" name="Chart"/>
<c:cNvChartSpPr/>
<c:nvPr/>
</c:nvChartSpPr>
<c:chartSpace>
<c:chart>
<c:title>
<c:tx>
<c:rich>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" sz="1800"/>
<a:t>{}</a:t>
</a:r>
</a:p>
</c:rich>
</c:tx>
</c:title>
<c:plotArea>
<c:layout/>
<c:lineChart>
<c:grouping val="lineMarkers"/>"#,
        shape_id,
        shape_id,
        chart.x,
        chart.y,
        chart.width,
        chart.height,
        escape_xml(&chart.title)
    );

    // Add series
    for (idx, series) in chart.series.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:ser>
<c:idx val="{}"/>
<c:order val="{}"/>
<c:title>
<c:tx>
<c:rich>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" sz="1000"/>
<a:t>{}</a:t>
</a:r>
</a:p>
</c:rich>
</c:tx>
</c:title>
<c:dLbls>
<c:showVal val="0"/>
</c:dLbls>
<c:val>
<c:numRef>
<c:f>Sheet1!$B${}:$B${}</c:f>
<c:numCache>
<c:formatCode>General</c:formatCode>"#,
            idx,
            idx,
            escape_xml(&series.name),
            2 + idx,
            2 + idx + chart.series.len()
        ));

        for value in &series.values {
            xml.push_str(&format!(
                r#"
<c:pt idx="0">
<c:v>{}</c:v>
</c:pt>"#,
                value
            ));
        }

        xml.push_str(
            r#"
</c:numCache>
</c:numRef>
</c:val>
</c:ser>"#
        );
    }

    // Add axes
    xml.push_str(
        r#"
<c:catAx>
<c:axId val="1"/>
<c:scaling>
<c:orientation val="minMax"/>
</c:scaling>
<c:delete val="0"/>
<c:axPos val="b"/>
<c:majorGridlines/>
<c:numFmt formatCode="General" sourceLinked="1"/>
<c:tickLblPos val="low"/>
<c:crossAx val="2"/>
<c:crosses val="autoZero"/>
<c:strRef>
<c:f>Sheet1!$A$2:$A$"#
    );

    xml.push_str(&format!("{}", 1 + chart.category_count()));
    xml.push_str(
        r#"</c:f>
<c:strCache>
<c:ptCount val=""#
    );

    xml.push_str(&format!("{}", chart.category_count()));
    xml.push_str("\">");

    for (idx, cat) in chart.categories.iter().enumerate() {
        xml.push_str(&format!(
            r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
            idx, escape_xml(cat)
        ));
    }

    xml.push_str(
        r#"
</c:strCache>
</c:strRef>
</c:catAx>
<c:valAx>
<c:axId val="2"/>
<c:scaling>
<c:orientation val="minMax"/>
</c:scaling>
<c:delete val="0"/>
<c:axPos val="l"/>
<c:majorGridlines/>
<c:numFmt formatCode="General" sourceLinked="1"/>
<c:tickLblPos val="low"/>
<c:crossAx val="1"/>
<c:crosses val="autoZero"/>
</c:valAx>
</c:lineChart>
</c:plotArea>
<c:legend>
<c:legendPos val="r"/>
<c:overlay val="0"/>
</c:legend>
<c:plotVisOnly val="1"/>
</c:chart>
</c:chartSpace>
</a:graphicData>
</a:graphic>
</p:graphicFrame>"#
    );

    xml
}

/// Generate pie chart XML
fn generate_pie_chart_xml(chart: &Chart, shape_id: usize) -> String {
    let mut xml = format!(
        r#"<p:graphicFrame>
<p:nvGraphicFramePr>
<p:cNvPr id="{}" name="Chart {}"/>
<p:cNvGraphicFramePr/>
<p:nvPr/>
</p:nvGraphicFramePr>
<p:xfrm>
<a:off x="{}" y="{}"/>
<a:ext cx="{}" cy="{}"/>
</p:xfrm>
<a:graphic>
<a:graphicData uri="http://schemas.openxmlformats.org/drawingml/2006/chart">
<c:chartSpace xmlns:c="http://schemas.openxmlformats.org/drawingml/2006/chart" xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
<c:nvChartSpPr>
<c:cNvPr id="1" name="Chart"/>
<c:cNvChartSpPr/>
<c:nvPr/>
</c:nvChartSpPr>
<c:chartSpace>
<c:chart>
<c:title>
<c:tx>
<c:rich>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" sz="1800"/>
<a:t>{}</a:t>
</a:r>
</a:p>
</c:rich>
</c:tx>
</c:title>
<c:plotArea>
<c:layout/>
<c:pieChart>
<c:varyColors val="1"/>"#,
        shape_id,
        shape_id,
        chart.x,
        chart.y,
        chart.width,
        chart.height,
        escape_xml(&chart.title)
    );

    // For pie chart, use first series only
    if let Some(series) = chart.series.first() {
        xml.push_str(&format!(
            r#"
<c:ser>
<c:idx val="0"/>
<c:order val="0"/>
<c:title>
<c:tx>
<c:rich>
<a:bodyPr/>
<a:lstStyle/>
<a:p>
<a:r>
<a:rPr lang="en-US" sz="1000"/>
<a:t>{}</a:t>
</a:r>
</a:p>
</c:rich>
</c:tx>
</c:title>
<c:dLbls>
<c:showCatName val="1"/>
<c:showPercent val="1"/>
</c:dLbls>
<c:val>
<c:numRef>
<c:f>Sheet1!$B$2:$B${}</c:f>
<c:numCache>
<c:formatCode>General</c:formatCode>"#,
            escape_xml(&series.name),
            1 + series.values.len()
        ));

        for (idx, value) in series.values.iter().enumerate() {
            xml.push_str(&format!(
                r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
                idx, value
            ));
        }

        xml.push_str(
            r#"
</c:numCache>
</c:numRef>
</c:val>
<c:cat>
<c:strRef>
<c:f>Sheet1!$A$2:$A$"#
        );

        xml.push_str(&format!("{}", 1 + chart.category_count()));
        xml.push_str(
            r#"</c:f>
<c:strCache>
<c:ptCount val=""#
        );

        xml.push_str(&format!("{}", chart.category_count()));
        xml.push_str("\">");

        for (idx, cat) in chart.categories.iter().enumerate() {
            xml.push_str(&format!(
                r#"
<c:pt idx="{}">
<c:v>{}</c:v>
</c:pt>"#,
                idx, escape_xml(cat)
            ));
        }

        xml.push_str(
            r#"
</c:strCache>
</c:strRef>
</c:cat>
</c:ser>"#
        );
    }

    xml.push_str(
        r#"
</c:pieChart>
</c:plotArea>
<c:legend>
<c:legendPos val="r"/>
<c:overlay val="0"/>
</c:legend>
<c:plotVisOnly val="1"/>
</c:chart>
</c:chartSpace>
</a:graphicData>
</a:graphic>
</p:graphicFrame>"#
    );

    xml
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace("'", "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::charts::{Chart, ChartSeries};

    #[test]
    fn test_generate_bar_chart_xml() {
        let chart = Chart::new(
            "Sales",
            ChartType::Bar,
            vec!["Q1".to_string(), "Q2".to_string()],
            0,
            0,
            5000000,
            3750000,
        )
        .add_series(ChartSeries::new("2024", vec![100.0, 150.0]));

        let xml = generate_bar_chart_xml(&chart, 1);
        assert!(xml.contains("barChart"));
        assert!(xml.contains("Sales"));
        assert!(xml.contains("2024"));
    }

    #[test]
    fn test_generate_line_chart_xml() {
        let chart = Chart::new(
            "Trend",
            ChartType::Line,
            vec!["Jan".to_string(), "Feb".to_string()],
            0,
            0,
            5000000,
            3750000,
        )
        .add_series(ChartSeries::new("Revenue", vec![1000.0, 1200.0]));

        let xml = generate_line_chart_xml(&chart, 1);
        assert!(xml.contains("lineChart"));
        assert!(xml.contains("Trend"));
    }

    #[test]
    fn test_generate_pie_chart_xml() {
        let chart = Chart::new(
            "Distribution",
            ChartType::Pie,
            vec!["A".to_string(), "B".to_string(), "C".to_string()],
            0,
            0,
            5000000,
            3750000,
        )
        .add_series(ChartSeries::new("Data", vec![30.0, 50.0, 20.0]));

        let xml = generate_pie_chart_xml(&chart, 1);
        assert!(xml.contains("pieChart"));
        assert!(xml.contains("Distribution"));
    }

    #[test]
    fn test_escape_xml_in_chart() {
        let chart = Chart::new(
            "Test & <Data>",
            ChartType::Bar,
            vec!["A".to_string()],
            0,
            0,
            1000000,
            1000000,
        );

        let xml = generate_bar_chart_xml(&chart, 1);
        assert!(xml.contains("&amp;"));
        assert!(xml.contains("&lt;"));
        assert!(xml.contains("&gt;"));
    }
}
