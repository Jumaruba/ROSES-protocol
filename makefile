.PHONY: graph clean
all: test graph code

target = test_rnd_1x1_noseq

test:
	@echo "Running test..."
	@cargo test $(target) > output

graph:
	@echo "Cleaning output..."
	@cat output | \awk 's{s=s"\n"$$0;} />> BEGIN/{s=$$0;} END{print s;}' > output_1
	
	@echo "Getting graph..."
	@sed -e '/^--/!d' output_1  > output_2
	@sed 's/-- //' output_2 > output_3

	@echo "Cleaning garbage and moving to template"
	@echo "sequenceDiagram" > diagram.mermaid
	@cat output_3 >> diagram.mermaid
	@rm output_*

code:
	@echo "Getting wrong test..."
	@cat output | \awk 's{s=s"\n"$$0;} />> BEGIN/{s=$$0;} END{print s;}' > output_1

	@echo "Getting code..."
	@sed -e '/^++/!d' output_1  > output_2
	@sed 's/++ //' output_2 > output_3 

	@echo "Cleaning garbage and moving to template"
	@echo "use handoff_register::{handoff::Handoff, types::NodeId};" > code.rs
	@echo "#[test]" >> code.rs 
	@echo "pub fn code(){" >> code.rs
	@cat output_3 >> code.rs 
	@echo "assert_eq!(true, false);" >> code.rs
	@echo "}" >> code.rs
	@rm output_*
	@mv code.rs ./tests/

run_code:
	@echo "Running code..."
	@cargo test code > output_code 
	@echo "Generated output_code file"

clean: 
	@rm diagram.mermaid output ./tests/code.rs



