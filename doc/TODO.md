stdf 
    endian -i tests/test_data/test.stdf

    show
    list 
        records
        types

    create -i tests/test_data/test.stdf -o ... (-t 3.14 -p) 
    duplicate

    is
        ws -i tests/test_data/test.stdf
        ft -i tests/test_data/test.stdf
        be -i tests/test_data/test.stdf
        le -i tests/test_data/test.stdf
        clean -i tests/test_data/test.stdf --> ends with MRR
        finished
        retest -i tests/test_data/test.stdf
        concatenable -i file1.stdf -i file2.stdf

    count 
        records -i tests/test_data/test.stdf -v 
        parts -i tests/test_data/test.stdf
        yield -i tests/test_data/test.stdf 
        tests -i tests/test_data/test.stdf -v
        sites -i tests/test_data/test.stdf
        heads -i tests/test_data/test.stdf
        sbin -i tests/test_data/test.stdf -v
        hbin -i tests/test_data/test.stdf -v

    dump
        record -i tests/test_data/test.stdf --offset 12345
        records -i tests/test_data/test.stdf -r FAR MIR ...
        info -i tests/test_data/test.stdf --> mir & sdr
        index -i tests/test_data/test.stdf 
        length -i tests/test_data/test.stdf --> returns the length of the file

    to 
        csv -i tests/test_data/test.stdf (-o ...) -p
        xlsx -i tests/test_data/test.stdf (-o ...) -p
        be -i tests/test_data/test.stdf (-o ...) -p
        le -i tests/test_data/test.stdf (-o ...) -p
        npy -i tests/test_data/test.stdf (-o ...) -p
        hdf5 -i tests/test_data/test.stdf (-o ...) -p
        atdf -i tests/files/test.stdf (-o ...) -p

    strip -i tests/test_data/test.stdf (-o ...) --atr --dtr --gdr --pcr --sbr --hbr --id

    report -i tests/test_data/test.stdf -p  --> writes a pdf

    repair -i tests/test_data/test.stdf (-o -p) 

    concat -i file1.stdf file2.stdf -o concat.stdf -p

    deflate -i tests/test_data/test.stdf --gzip -p
                                         --lzma -p
                                         --zip -p
                                         --xz -p
                                         --bzip2 -p
                                         --lz4 -p
                                         --zst -p


    inflate -i tests/test_data/test.stdf.gzip -p
