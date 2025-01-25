stdf 
    endian -i tests/test_data/test.stdf
    
    yield -i tests/test_data/test.stdf -v -f

    show
    list 
        records
        types

    create -i tests/test_data/test.stdf -o ... (-t 3.14 -p) 
    duplicate

    is
        ws -i tests/test_data/test.stdf
        ft -i tests/test_data/test.stdf
        clean -i tests/test_data/test.stdf --> ends with MRR
        retest -i tests/test_data/test.stdf

    count 
        records -i tests/test_data/test.stdf -v -r FAR MIR ...
        parts -i tests/test_data/test.stdf -v
        tests -i tests/test_data/test.stdf [-v?]
        sites -i tests/test_data/test.stdf
        heads -i tests/test_data/test.stdf

    strip -i tests/test_data/test.stdf --atr --dtr --gdr --pcr --sbr --hbr 

    repair -i tests/test_data/test.stdf

    concat 

    dump
        records -i tests/test_data/test.stdf -r FAR MIR ...
        info -i tests/test_data/test.stdf --> mir & sdr

    to 
        csv -i tests/test_data/test.stdf (-o ...) -p
        xlsx -i tests/test_data/test.stdf (-o ...) -p
        be -i tests/test_data/test.stdf (-o ...) -p
        le -i tests/test_data/test.stdf (-o ...) -p
        npy -i tests/test_data/test.stdf (-o ...) -p
        hdf5 -i tests/test_data/test.stdf (-o ...) -p

    deflate -i tests/test_data/test.stdf --gzip -p
                                         --lzma -p
                                         --zip -p
                                         --xz -p
                                         --bzip2 -p
                                         --lz4 -p
                                         --zst -p


    inflate -i tests/test_data/test.stdf.gzip -p
