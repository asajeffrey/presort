EXPERIMENTS = shape_edit additions removals shape_chance data_edit data_incr resort_chance data_batches
DATAFILES = $(addprefix ../target/data/, $(addsuffix .data, $(EXPERIMENTS)))
PDFFILES = $(addprefix ../target/data/, $(addsuffix .pdf, $(EXPERIMENTS)))

../target/data/%.data: bench_%.sh
	bash $(abspath $<)

../target/data/%.pdf: bench_%.sh ../target/data/%.data
	bash $(abspath $<) --onlyplot

paper.pdf: paper.tex paper.bib $(PDFFILES)
	pdflatex $<
	bibtex $(basename $<)
	pdflatex $<
	pdflatex $<
