make a better .gitignore
have a look at [rust-stdf](https://github.com/noonchen/rust-stdf/tree/main)

stdf 
    endian -i tests/fixtures/test.stdf

    show
    list 
        records
        types

    create -i tests/fixtures/test.stdf -o ... (-t 3.14 -p) 
    duplicate
    generate

    anonymize -i test/files/test.stdf -d --> delete original afterwards

    is
        ws -i tests/fixtures/test.stdf -v
        ft -i tests/fixtures/test.stdf -v
        be -i tests/fixtures/test.stdf -v
        le -i tests/fixtures/test.stdf -v
        clean -i tests/fixtures/test.stdf -v --> ends with MRR
        finished
        terminated
        ready
        retest -i tests/fixtures/test.stdf
        concatenable -i file1.stdf -i file2.stdf

    count 
    tally
        records -i tests/fixtures/test.stdf -v 
        parts -i tests/fixtures/test.stdf
        yield -i tests/fixtures/test.stdf 
        tests -i tests/fixtures/test.stdf -v
        sites -i tests/fixtures/test.stdf
        heads -i tests/fixtures/test.stdf
        parallelism -i tests/fixtures/test.stdf 
        sbin -i tests/fixtures/test.stdf -v
        hbin -i tests/fixtures/test.stdf -v

    dump
        record -i tests/fixtures/test.stdf --offset 12345
        records -i tests/fixtures/test.stdf -r FAR MIR ...
        info -i tests/fixtures/test.stdf --> mir & sdr
        index -i tests/fixtures/test.stdf 
        length -i tests/fixtures/test.stdf --> returns the length of the file

    to 
        csv -i tests/fixtures/test.stdf (-o ...) -p
        xlsx -i tests/fixtures/test.stdf (-o ...) -p
        be -i tests/fixtures/test.stdf (-o ...) -p
        le -i tests/fixtures/test.stdf (-o ...) -p
        npy -i tests/fixtures/test.stdf (-o ...) -p
        hdf5 -i tests/fixtures/test.stdf (-o ...) -p
        atdf -i tests/fixtures/test.stdf (-o ...) -p
        metis -i tests/fixtures/test.stdf -p 

    strip -i tests/fixtures/test.stdf (-o ...) --atr --dtr --gdr --pcr --sbr --hbr --id

    report -i tests/fixtures/test.stdf -p  --> writes a pdf

    repair -i tests/fixtures/test.stdf (-o -p) 

    concat -i file1.stdf file2.stdf -o concat.stdf -p

    deflate -i tests/fixtures/test.stdf --gzip -p
                                         --lzma -p
                                         --zip -p
                                         --xz -p
                                         --bz2 -p
                                         --lz4 -p
                                         --zst -p ---> zstd crate
                                         --7z -p ---> sevenz_rust crate


    inflate -i tests/fixtures/test.stdf.gzip -p

    analyze -i tests/fixtures/test.std (-o ...) -p 
