# Call Functions

This project tests function calls and structured data extraction from small LLM models available in Ollama, as well as translation capabilities.

## Overview

The project evaluates different Ollama models on two main tasks:
1. **Structured Data Extraction** - Extracting structured information (names, ages, professions) from text
2. **Language Translation** - Translating text from English to Russian

## Prerequisites

- Rust and Cargo installed
- Ollama running locally
- Required models pulled to Ollama

## Required Models

Before running the tests, pull these models to Ollama:

```bash
ollama pull functiongemma:latest
ollama pull translategemma:latest
```

## Quick Start

```bash
cargo run
```

This will run both extraction and translation tests with the default models.

## Configuration

The project uses two main models:
- `functiongemma:latest` - for structured data extraction tests
- `translategemma:latest` - for translation tests

You can modify the models in `src/main.rs` by updating the `get_models()` and `get_translation_models()` functions.

## Test Data

- **Extraction tests**: Located in `extract_persons.json` with 10 complex prompts
- **Translation tests**: Located in `translate_examples.json` with 10 English-Russian translation pairs

## Output

After running the tests, you'll find CSV files with statistics in the `data/` directory:
- `data/extract/person_statistics.csv` - Results for data extraction tests
- `data/extract/company_statistics.csv` - Results for complex data extraction tests
- `data/translate/translate_statistics.csv` - Results for translation tests

Each CSV contains:
- Model name
- Total test count
- Successful extraction/translation count
- Average processing time
- Individual test results with prompts, responses, validity, and processing time

## Customization

To test different models:
1. Update the model names in `src/main.rs`
2. Ensure the models are pulled to Ollama
3. Run `cargo run`

To add more test cases:
1. Modify `extract_persons.json` for extraction tests
2. Modify `translate_examples.json` for translation tests

## Understanding Results

- **Success rate**: Percentage of tests that matched expected results
- **Processing time**: Time taken for each test in milliseconds
- **Validity**: Whether the extracted/translated result matched the expected value
- **Response**: The actual output from the model

Note: Translation tests may show lower success rates due to semantic differences between translations, even when the meaning is preserved.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.