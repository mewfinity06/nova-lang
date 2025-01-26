# Grammar

T = _ program end_of_file;

program = _
        | program decleration
        ;

decleration = var_decl
            | statement
            ;
