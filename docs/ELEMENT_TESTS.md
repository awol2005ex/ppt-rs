# PowerPoint Element Tests

Comprehensive test suite for different PowerPoint elements and features.

## Overview

The `tests/pptx_elements_test.rs` file contains 31 tests covering various PowerPoint slide elements:
- Slide content types
- Font sizes
- Bold formatting
- Text content variations
- Presentation structures
- Special characters and Unicode

## Test Categories

### 1. Slide Content Tests (7 tests)

Tests for different types of slide content:

#### test_slide_with_title_only
- **Purpose**: Verify slides with only a title, no content
- **Generated File**: test_title_only_slides.pptx
- **Validates**: Title-only slide structure

#### test_slide_with_single_bullet
- **Purpose**: Test slide with one bullet point
- **Validates**: Single bullet handling

#### test_slide_with_multiple_bullets
- **Purpose**: Test slide with 5 bullet points
- **Generated File**: test_single_bullet.pptx
- **Validates**: Multiple bullet point handling

#### test_slide_with_long_text
- **Purpose**: Test handling of long text in bullets
- **Generated File**: test_long_text.pptx
- **Validates**: Text wrapping and overflow handling

#### test_slide_with_special_characters
- **Purpose**: Test special XML characters (&, <, >, ", ')
- **Generated File**: test_special_characters.pptx
- **Validates**: XML escaping

#### test_slide_with_empty_bullets
- **Purpose**: Test handling of empty bullet points
- **Validates**: Empty string handling

#### test_slide_with_unicode_text
- **Purpose**: Test Unicode text (Japanese, Chinese, Russian)
- **Generated File**: test_unicode.pptx
- **Validates**: Unicode support

### 2. Font Size Tests (4 tests)

Tests for different font size configurations:

#### test_slide_with_small_fonts
- **Purpose**: Test small font sizes (18pt title, 12pt content)
- **Validates**: Minimum font size handling

#### test_slide_with_large_fonts
- **Purpose**: Test large font sizes (72pt title, 48pt content)
- **Validates**: Large font size handling

#### test_slide_with_extreme_font_sizes
- **Purpose**: Test extreme sizes (8pt title, 96pt content)
- **Validates**: Edge case font sizes

#### test_slide_with_matching_font_sizes
- **Purpose**: Test when title and content have same size
- **Validates**: Equal font size handling

### 3. Bold Formatting Tests (4 tests)

Tests for bold text formatting:

#### test_slide_with_bold_title
- **Purpose**: Test bold title with regular content
- **Validates**: Title-only bold

#### test_slide_with_bold_content
- **Purpose**: Test regular title with bold content
- **Validates**: Content-only bold

#### test_slide_with_all_bold
- **Purpose**: Test both title and content bold
- **Generated File**: test_bold_variations.pptx
- **Validates**: All bold formatting

#### test_slide_with_no_bold
- **Purpose**: Test no bold formatting
- **Validates**: Regular (non-bold) text

### 4. Combined Formatting Tests (2 tests)

Tests for multiple formatting options together:

#### test_slide_with_all_formatting
- **Purpose**: Test all formatting options combined
- **Validates**: Font size + bold combinations

#### test_slide_with_mixed_formatting
- **Purpose**: Test mixed formatting (some bold, some not)
- **Validates**: Selective formatting

### 5. Presentation Structure Tests (3 tests)

Tests for different presentation sizes:

#### test_single_slide_presentation
- **Purpose**: Test single-slide presentation
- **Generated File**: test_minimal.pptx
- **Validates**: Minimal presentation

#### test_two_slide_presentation
- **Purpose**: Test two-slide presentation
- **Validates**: Multi-slide structure

#### test_ten_slide_presentation
- **Purpose**: Test 10-slide presentation
- **Validates**: Larger presentations

#### test_presentation_with_varied_content
- **Purpose**: Test presentation with different slide styles
- **Validates**: Mixed content across slides

### 6. PPTX Generation Tests (11 tests)

Tests that generate actual PPTX files:

#### test_generate_title_only_slides
- **Output**: test_title_only_slides.pptx (7.8K)
- **Content**: 3 slides with titles only
- **Use Case**: Title-only presentations

#### test_generate_single_bullet_slides
- **Output**: test_single_bullet.pptx (7.0K)
- **Content**: 2 slides with single bullet each
- **Use Case**: Minimal content presentations

#### test_generate_many_bullets_slide
- **Output**: test_many_bullets.pptx (6.0K)
- **Content**: 1 slide with 10 bullet points
- **Use Case**: Dense content slides

#### test_generate_long_text_slide
- **Output**: test_long_text.pptx (6.1K)
- **Content**: Slides with long text bullets
- **Use Case**: Extended text handling

#### test_generate_special_characters_slide
- **Output**: test_special_characters.pptx (6.0K)
- **Content**: Special characters (&, <, >, ", ')
- **Use Case**: XML escaping verification

#### test_generate_unicode_slide
- **Output**: test_unicode.pptx (6.1K)
- **Content**: Multiple languages (English, Japanese, Chinese, Korean, Russian)
- **Use Case**: International text support

#### test_generate_font_size_variations
- **Output**: test_font_variations.pptx (10K)
- **Content**: 5 slides with different font sizes (12pt to 80pt)
- **Use Case**: Font size range demonstration

#### test_generate_bold_variations
- **Output**: test_bold_variations.pptx (9.0K)
- **Content**: 4 slides with different bold combinations
- **Use Case**: Bold formatting variations

#### test_generate_comprehensive_presentation
- **Output**: test_comprehensive.pptx (10K)
- **Content**: 5-slide professional presentation
- **Use Case**: Real-world presentation example

#### test_generate_minimal_presentation
- **Output**: test_minimal.pptx (5.9K)
- **Content**: Single slide with title only
- **Use Case**: Smallest possible presentation

## Generated Files Summary

| File | Size | Slides | Purpose |
|------|------|--------|---------|
| test_title_only_slides.pptx | 7.8K | 3 | Title-only slides |
| test_single_bullet.pptx | 7.0K | 2 | Single bullet per slide |
| test_many_bullets.pptx | 6.0K | 1 | 10 bullet points |
| test_long_text.pptx | 6.1K | 2 | Extended text |
| test_special_characters.pptx | 6.0K | 1 | XML special chars |
| test_unicode.pptx | 6.1K | 1 | Multiple languages |
| test_font_variations.pptx | 10K | 5 | Font size range |
| test_bold_variations.pptx | 9.0K | 4 | Bold combinations |
| test_comprehensive.pptx | 10K | 5 | Professional presentation |
| test_minimal.pptx | 5.9K | 1 | Minimal content |

## Running Tests

### Run all element tests
```bash
cargo test --test pptx_elements_test
```

### Run specific test
```bash
cargo test test_generate_unicode_slide
```

### Run with output
```bash
cargo test --test pptx_elements_test -- --nocapture
```

## Verifying Generated Files

### Check file format
```bash
file target/test_output/test_unicode.pptx
```

### Check slide content
```bash
unzip -p target/test_output/test_unicode.pptx ppt/slides/slide1.xml | grep -o '<a:t>[^<]*</a:t>'
```

### Check font sizes
```bash
unzip -p target/test_output/test_font_variations.pptx ppt/slides/slide1.xml | grep -o 'sz="[0-9]*"'
```

### Check bold formatting
```bash
unzip -p target/test_output/test_bold_variations.pptx ppt/slides/slide1.xml | grep -o 'b="[01]"'
```

## Test Coverage

### Content Types
- ✅ Title-only slides
- ✅ Single bullet points
- ✅ Multiple bullet points (up to 10)
- ✅ Long text content
- ✅ Special characters
- ✅ Unicode text (4 languages)
- ✅ Empty bullets

### Formatting
- ✅ Font sizes (8pt to 96pt)
- ✅ Bold title
- ✅ Bold content
- ✅ Combined formatting
- ✅ Mixed formatting

### Presentation Sizes
- ✅ Single slide
- ✅ Two slides
- ✅ Ten slides
- ✅ Five slides (comprehensive)

## Quality Metrics

**Total Tests**: 31
- Content tests: 7
- Font size tests: 4
- Bold formatting tests: 4
- Combined formatting tests: 2
- Presentation structure tests: 3
- PPTX generation tests: 11

**All tests passing**: ✅

**Generated files**: 15 PPTX files
**Total size**: ~130KB

## Use Cases

### Testing Text Handling
- Use `test_unicode.pptx` for international text
- Use `test_special_characters.pptx` for XML escaping
- Use `test_long_text.pptx` for text wrapping

### Testing Formatting
- Use `test_font_variations.pptx` for font size ranges
- Use `test_bold_variations.pptx` for bold combinations
- Use `test_comprehensive.pptx` for mixed formatting

### Testing Structure
- Use `test_minimal.pptx` for minimal presentations
- Use `test_many_bullets.pptx` for dense content
- Use `test_comprehensive.pptx` for complex structures

## Troubleshooting

### Files not generated
```bash
cargo test --test pptx_elements_test
ls target/test_output/
```

### Verify file integrity
```bash
file target/test_output/test_unicode.pptx
unzip -t target/test_output/test_unicode.pptx
```

### Check specific content
```bash
unzip -p target/test_output/test_unicode.pptx ppt/slides/slide1.xml | xmllint --format -
```

## See Also

- [TESTING.md](TESTING.md) - General testing guide
- [TEST_GENERATION.md](TEST_GENERATION.md) - Test file generation
- [ADVANCED_FEATURES.md](ADVANCED_FEATURES.md) - Feature documentation
