
.PHONY: all clean

PACKAGES := bootstrap-config bootstrap-config-types bootstrap-macros min_test_project config_tests bootstrap-config-builder-core build_helper config_core config_macros stage0_parser_crate

all: $(patsubst %,.shear_processed_%,$(PACKAGES))

.shear_processed_%:
	cargo shear --fix --expand -p $*
	touch $@

clean:
	rm -f .shear_processed_*
