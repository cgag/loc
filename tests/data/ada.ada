 procedure scan_for_file_ghosts (
    directory_path   : in String;
    file_ghosts      : out Filename_Container.Vector;
    directory_ghosts : out Filename_Container.Vector);
 --  Given an existing directory path, the procedure will produce
 --  both a list of deleted directories and a list of files deleted
 --  (but with available version) from that directory
