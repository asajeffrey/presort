EXPERIMENTS = chance
DATAFILES = $(addprefix ../target/data/, $(addsuffix .data, $(EXPERIMENTS)))
PDFFILES = $(addprefix ../target/data/, $(addsuffix .pdf, $(EXPERIMENTS)))

../target/data/%.data: %_bench.sh
	cd .. ; bash $(abspath $<)

../target/data/%.pdf: %_plot.sh ../target/data/%.data
	cd .. ; bash $(abspath $<)

paper.pdf: paper.tex paper.bib $(PDFFILES)
	pdflatex $<
	bibtex $(basename $<)
	pdflatex $<
	pdflatex $<
