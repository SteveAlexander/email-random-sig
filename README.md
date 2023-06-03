# email-sigs

MacOS command linen app to maintain a collection of email signatures in the MacOS Mail application.

Signatures are updated for all accounts, with a prefixed id.
The default prefix is <Autoquote >, but this can be overridden.

    Usage:
      mail-sigs -h | --help
      mail-sigs --prefix "custom prefix "
      mail-sigs FILENAME1 FILENAME2 FILENAME3
      mail-sigs --prefix "custom prefix " FILENAME1 FILENAME2 FILENAME3
      mail-sigs --erase-all  # erases signatures with default prefix 
      mail-sigs --prefix "custom prefix " --erase-all
