FC = gfortran
AR = ar
AFLAG = crus
SFlAG = -shared -o
FFLAGS = -O3 -Wall -fexceptions -g -Wno-uninitialized -c -fPIC -o
CGFAM  = cgfam.f
BLAS    = blas.f
TIMER   = timer.f
CGSEARCH = cgsearch.f
static : 
	@echo "static linking selected"
	@echo "Compiling"
	$(FC) $(CGFAM) $(FFLAGS) $(OUTPUT)/cgfam.o
	$(FC) $(CGSEARCH) $(FFLAGS) $(OUTPUT)/cgsearch.o
	$(FC) $(BLAS) $(FFLAGS) $(OUTPUT)/blas.o
	$(FC) $(TIMER) $(FFLAGS) $(OUTPUT)/timer.o
	@echo "creating static library"
	$(AR) $(AFLAG) $(OUTPUT)/libcgfam.a $(OUTPUT)/*.o
dylib :
ifeq ($(OSNAME),Windows)
	@echo "dynamic linking selected"
	@echo "Compiling"
	$(FC) $(CGFAM) $(FFLAGS) $(OUTPUT)/cgfam.o
	$(FC) $(CGSEARCH) $(FFLAGS) $(OUTPUT)/cgsearch.o
	$(FC) $(BLAS) $(FFLAGS) $(OUTPUT)/blas.o
	$(FC) $(TIMER) $(FFLAGS) $(OUTPUT)/timer.o
	@echo "creating static library"
	$(FC) $(SFlAG) $(OUTPUT)/libcgfam.dll $(OUTPUT)/*.o
endif
ifeq ($(OSNAME),Linux)
	@echo "dynamic linking selected"
	@echo "Compiling"
	@echo $(OUTPUT)
	$(FC) $(CGFAM) $(FFLAGS) $(OUTPUT)/cgfam.o
	$(FC) $(CGSEARCH) $(FFLAGS) $(OUTPUT)/cgsearch.o
	$(FC) $(BLAS) $(FFLAGS) $(OUTPUT)/blas.o
	$(FC) $(TIMER) $(FFLAGS) $(OUTPUT)/timer.o
	@echo "creating static library"
	$(FC) $(SFlAG) $(OUTPUT)/libcgfam.so $(OUTPUT)/*.o
endif
ifeq ($(OSNAME),Macos)
	@echo "dynamic linking selected"
	@echo "Compiling"
	$(FC) $(CGFAM) $(FFLAGS) $(OUTPUT)/cgfam.o
	$(FC) $(CGSEARCH) $(FFLAGS) $(OUTPUT)/cgsearch.o
	$(FC) $(BLAS) $(FFLAGS) $(OUTPUT)/blas.o
	$(FC) $(TIMER) $(FFLAGS) $(OUTPUT)/timer.o
	@echo "creating static library"
	$(FC) $(SFlAG) $(OUTPUT)/libcgfam.dylib $(OUTPUT)/*.o
endif