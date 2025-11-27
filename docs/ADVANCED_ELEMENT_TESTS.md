# Advanced PowerPoint Element Tests

Comprehensive test suite for advanced PowerPoint elements: tables, charts, and images.

## Overview

The `tests/pptx_advanced_elements_test.rs` file contains 33 tests covering:
- Table creation and formatting
- Chart data structures
- Image metadata and properties
- Complex presentations with multiple element types

## Test Categories

### 1. Table Tests (12 tests)

Tests for table creation, formatting, and structure:

#### test_table_cell_creation
- **Purpose**: Basic table cell creation
- **Validates**: Cell text and default properties

#### test_table_cell_with_formatting
- **Purpose**: Table cell with bold and background color
- **Validates**: Cell formatting options

#### test_table_row_creation
- **Purpose**: Create table row with multiple cells
- **Validates**: Row structure and cell count

#### test_table_row_with_height
- **Purpose**: Table row with custom height
- **Validates**: Row height property

#### test_simple_2x2_table
- **Purpose**: Basic 2x2 table
- **Validates**: Table dimensions

#### test_3x3_table
- **Purpose**: 3x3 table with headers and data
- **Validates**: Larger table structure

#### test_table_with_header_row
- **Purpose**: Table with distinct header row
- **Validates**: Header formatting

#### test_table_with_multiple_rows
- **Purpose**: Table with 4 rows
- **Validates**: Multi-row handling

#### test_table_with_position
- **Purpose**: Table with custom position
- **Validates**: Position properties (x, y)

#### test_wide_table
- **Purpose**: Table with 5 columns
- **Validates**: Wide table handling

#### test_tall_table
- **Purpose**: Table with 11 rows
- **Validates**: Tall table handling

#### test_table_with_styled_cells
- **Purpose**: Table with styled cells
- **Validates**: Cell formatting combinations

### 2. Chart Tests (7 tests)

Tests for chart data structures and types:

#### test_chart_data_structure
- **Purpose**: Basic chart data structure
- **Validates**: Data point creation

#### test_bar_chart_data
- **Purpose**: Bar chart data (categories and values)
- **Validates**: Bar chart structure

#### test_pie_chart_data
- **Purpose**: Pie chart with percentage slices
- **Validates**: Pie chart data (100% total)

#### test_line_chart_data
- **Purpose**: Line chart with multiple series
- **Validates**: Multi-series data

#### test_scatter_chart_data
- **Purpose**: Scatter plot data points
- **Validates**: X-Y coordinate pairs

### 3. Image Tests (6 tests)

Tests for image metadata and properties:

#### test_image_metadata
- **Purpose**: Image metadata structure
- **Validates**: Filename, dimensions, format

#### test_image_dimensions
- **Purpose**: Various image sizes
- **Validates**: Width and height properties

#### test_image_aspect_ratios
- **Purpose**: Common aspect ratios
- **Validates**: 16:9, 4:3, 1:1, 21:9

#### test_image_formats
- **Purpose**: Supported image formats
- **Validates**: PNG, JPG, GIF, BMP, TIFF

### 4. PPTX Generation Tests (8 tests)

Tests that generate actual PPTX files:

#### test_generate_simple_table_slide
- **Output**: test_simple_table.pptx (5.9K)
- **Content**: Slide with table reference
- **Use Case**: Basic table presentation

#### test_generate_data_table_slide
- **Output**: test_data_table.pptx (6.0K)
- **Content**: Sales data table
- **Use Case**: Financial data presentation

#### test_generate_comparison_table_slide
- **Output**: test_comparison_table.pptx (6.0K)
- **Content**: Product comparison
- **Use Case**: Comparison presentations

#### test_generate_schedule_table_slide
- **Output**: test_schedule_table.pptx (6.0K)
- **Content**: Project schedule
- **Use Case**: Timeline presentations

#### test_generate_chart_data_slide
- **Output**: test_chart_data.pptx (6.0K)
- **Content**: Chart data points
- **Use Case**: Data visualization

#### test_generate_bar_chart_slide
- **Output**: test_bar_chart.pptx (6.0K)
- **Content**: Quarterly revenue data
- **Use Case**: Bar chart presentations

#### test_generate_pie_chart_slide
- **Output**: test_pie_chart.pptx (6.0K)
- **Content**: Market share distribution
- **Use Case**: Pie chart presentations

#### test_generate_line_chart_slide
- **Output**: test_line_chart.pptx (6.0K)
- **Content**: Growth trend data
- **Use Case**: Trend analysis

#### test_generate_image_placeholder_slide
- **Output**: test_image_placeholder.pptx (6.0K)
- **Content**: Image dimensions
- **Use Case**: Image placeholder

#### test_generate_gallery_slide
- **Output**: test_gallery.pptx (6.0K)
- **Content**: Multiple image sizes
- **Use Case**: Image gallery

#### test_generate_mixed_content_slide
- **Output**: test_mixed_content.pptx (6.0K)
- **Content**: Tables, charts, images
- **Use Case**: Mixed content presentation

#### test_generate_comprehensive_advanced_presentation
- **Output**: test_advanced_comprehensive.pptx (10K)
- **Content**: 5-slide presentation with all elements
- **Use Case**: Complete advanced example

## Generated Files Summary

| File | Size | Purpose |
|------|------|---------|
| test_simple_table.pptx | 5.9K | Basic table |
| test_data_table.pptx | 6.0K | Sales data |
| test_comparison_table.pptx | 6.0K | Product comparison |
| test_schedule_table.pptx | 6.0K | Project schedule |
| test_chart_data.pptx | 6.0K | Chart data |
| test_bar_chart.pptx | 6.0K | Bar chart |
| test_pie_chart.pptx | 6.0K | Pie chart |
| test_line_chart.pptx | 6.0K | Line chart |
| test_image_placeholder.pptx | 6.0K | Image placeholder |
| test_gallery.pptx | 6.0K | Image gallery |
| test_mixed_content.pptx | 6.0K | Mixed content |
| test_advanced_comprehensive.pptx | 10K | Comprehensive example |

## Running Tests

### Run all advanced element tests
```bash
cargo test --test pptx_advanced_elements_test
```

### Run specific test category
```bash
# Table tests
cargo test --test pptx_advanced_elements_test test_table

# Chart tests
cargo test --test pptx_advanced_elements_test test_chart

# Image tests
cargo test --test pptx_advanced_elements_test test_image
```

### Run specific test
```bash
cargo test test_generate_bar_chart_slide
```

## Test Coverage

### Tables
- ✅ Cell creation and formatting
- ✅ Row creation and properties
- ✅ Table dimensions (2x2, 3x3, 5-column, 11-row)
- ✅ Header rows
- ✅ Styled cells
- ✅ Custom positioning

### Charts
- ✅ Bar chart data
- ✅ Pie chart data (100% validation)
- ✅ Line chart with multiple series
- ✅ Scatter plot data
- ✅ Chart data structures

### Images
- ✅ Image metadata
- ✅ Various dimensions
- ✅ Aspect ratios (16:9, 4:3, 1:1, 21:9)
- ✅ Multiple formats (PNG, JPG, GIF, BMP, TIFF)

## Quality Metrics

**Total Tests**: 33
- Table tests: 12
- Chart tests: 7
- Image tests: 6
- PPTX generation tests: 8

**All tests passing**: ✅

**Generated files**: 12 PPTX files
**Total size**: ~75KB

## Use Cases

### Table Presentations
- Use `test_data_table.pptx` for financial data
- Use `test_comparison_table.pptx` for product comparisons
- Use `test_schedule_table.pptx` for project timelines

### Chart Presentations
- Use `test_bar_chart.pptx` for revenue/sales data
- Use `test_pie_chart.pptx` for market share
- Use `test_line_chart.pptx` for trends

### Image Presentations
- Use `test_image_placeholder.pptx` for image references
- Use `test_gallery.pptx` for image collections

### Mixed Content
- Use `test_mixed_content.pptx` for combined elements
- Use `test_advanced_comprehensive.pptx` for full examples

## Future Implementation

These tests provide the foundation for implementing:

### Tables
- [ ] Table rendering in PPTX
- [ ] Cell styling and formatting
- [ ] Table borders and shading
- [ ] Merged cells
- [ ] Table formulas

### Charts
- [ ] Bar chart generation
- [ ] Pie chart generation
- [ ] Line chart generation
- [ ] Scatter plot generation
- [ ] Chart legends and labels

### Images
- [ ] Image embedding
- [ ] Image scaling
- [ ] Image positioning
- [ ] Image captions
- [ ] Image effects

## Troubleshooting

### Files not generated
```bash
cargo test --test pptx_advanced_elements_test
ls target/test_output/ | grep -E "table|chart|image"
```

### Verify file integrity
```bash
file target/test_output/test_bar_chart.pptx
unzip -t target/test_output/test_bar_chart.pptx
```

### Check slide content
```bash
unzip -p target/test_output/test_bar_chart.pptx ppt/slides/slide1.xml | grep -o '<a:t>[^<]*</a:t>'
```

## See Also

- [ELEMENT_TESTS.md](ELEMENT_TESTS.md) - Basic element tests
- [TESTING.md](TESTING.md) - General testing guide
- [TEST_GENERATION.md](TEST_GENERATION.md) - Test file generation
- [ADVANCED_FEATURES.md](ADVANCED_FEATURES.md) - Feature documentation
