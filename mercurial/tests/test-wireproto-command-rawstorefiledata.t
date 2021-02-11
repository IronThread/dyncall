  $ . $TESTDIR/wireprotohelpers.sh
  $ hg init server
  $ enablehttpv2 server
  $ cd server
  $ echo a0 > a
  $ echo b0 > b
  $ hg -q commit -A -m 'commit 0'
  $ echo a1 > a
  $ hg commit -m 'commit 1'
  $ mkdir dir0
  $ mkdir dir1
  $ echo c0 > dir0/c
  $ echo d0 > dir0/d
  $ echo e0 > dir1/e
  $ echo f0 > dir1/f
  $ hg commit -A -m 'commit 2'
  adding dir0/c
  adding dir0/d
  adding dir1/e
  adding dir1/f
  $ echo f1 > dir1/f
  $ hg commit -m 'commit 3'

  $ hg serve -p $HGPORT -d --pid-file hg.pid -E error.log
  $ cat hg.pid > $DAEMON_PIDS

Missing requirement argument results in error

  $ sendhttpv2peer << EOF
  > command rawstorefiledata
  > EOF
  creating http peer for wire protocol version 2
  sending rawstorefiledata command
  abort: missing required arguments: files
  [255]

Unknown files value results in error

  $ sendhttpv2peer << EOF
  > command rawstorefiledata
  >     files eval:[b'unknown']
  > EOF
  creating http peer for wire protocol version 2
  sending rawstorefiledata command
  abort: unknown file type: unknown
  [255]

Requesting just changelog works

  $ sendhttpv2peer << EOF
  > command rawstorefiledata
  >     files eval:[b'changelog']
  > EOF
  creating http peer for wire protocol version 2
  sending rawstorefiledata command
  response: gen[
    {
      b'filecount': 1,
      b'totalsize': 527
    },
    {
      b'location': b'store',
      b'path': b'00changelog.i',
      b'size': 527
    },
    b'\x00\x01\x00\x01\x00\x00\x00\x00\x00\x00\x00@\x00\x00\x00?\x00\x00\x00\x00\x00\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff3\x90\xef\x85\x00s\xfb\xc2\xf0\xdf\xff"D4,\x8e\x92)\x01:\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00u992f4779029a3df8d0666d00bb924f69634e2641\ntest\n0 0\na\nb\n\ncommit 0\x00\x00\x00\x00\x00@\x00\x00\x00\x00\x00>\x00\x00\x00=\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\xff\xff\xff\xffD2\xd86&\xe8\xa9\x86U\xf0b\xec\x1f*C\xb0\x7f\x7f\xbb\xb0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00ua988fb43583e871d1ed5750ee074c6d840bbbfc8\ntest\n0 0\na\n\ncommit 1\x00\x00\x00\x00\x00~\x00\x00\x00\x00\x00N\x00\x00\x00W\x00\x00\x00\x02\x00\x00\x00\x02\x00\x00\x00\x01\xff\xff\xff\xff\xa4r\xd2\xea\x96U\x1a\x1e\xbb\x011-\xb2\xe6\xa7\x86\xd0F\x96o\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00x\x9c%\xc5\xc1\t\xc0 \x0c\x05\xd0{\xa6p\x03cjI\xd71\xf9\x11<H\xa1u\x7fJ\xf1]\x9eyu\x98\xa2\xb0Z\x88jk0\x11\x95z\xa0\xdb\x11\\\x81S\xfc*\xb4\xe2]\xc4\x89\t\xe3\xe1\xec;\xfc\x95\x1c\xbbN\xe4\xf7\x9cc%\xf9\x00S#\x19\x13\x00\x00\x00\x00\x00\xcc\x00\x00\x00\x00\x00C\x00\x00\x00B\x00\x00\x00\x03\x00\x00\x00\x03\x00\x00\x00\x02\xff\xff\xff\xff\x85kg{\x94a\x12i\xc5lW5[\x85\xf9\x95|\xfc\xc1\xb9\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00u90231ddca36fa178a0eed99bd03078112487dda3\ntest\n0 0\ndir1/f\n\ncommit 3',
    b''
  ]

Requesting just manifestlog works (as impractical as that operation may be).

  $ sendhttpv2peer << EOF
  > command rawstorefiledata
  >     files eval:[b'manifestlog']
  > EOF
  creating http peer for wire protocol version 2
  sending rawstorefiledata command
  response: gen[
    {
      b'filecount': 1,
      b'totalsize': 584
    },
    {
      b'location': b'store',
      b'path': b'00manifest.i',
      b'size': 584
    },
    b'\x00\x03\x00\x01\x00\x00\x00\x00\x00\x00\x00I\x00\x00\x00V\x00\x00\x00\x00\x00\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x99/Gy\x02\x9a=\xf8\xd0fm\x00\xbb\x92OicN&A\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00x\x9c\r\xca\xc1\x11\x00!\x08\x040\xdfV\x03+\xa2\x94\xb3\x8c\xd0\x7f\twy\x87\x03i\x95r\x96F6\xe5\x1c\x9a\x10-\x16\xba|\x07\xab\xe5\xd1\xf08s\\\x8d\xc2\xbeo)w\xa9\x8b;\xa2\xff\x95\x19\x02jB\xab\x0c\xea\xf3\x03\xcf\x1d\x16\t\x00\x00\x00\x00\x00I\x00\x00\x00\x00\x007\x00\x00\x00V\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\xff\xff\xff\xff\xa9\x88\xfbCX>\x87\x1d\x1e\xd5u\x0e\xe0t\xc6\xd8@\xbb\xbf\xc8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00+\x00\x00\x00+a\x009a38122997b3ac97be2a9aa2e556838341fdf2cc\n\x00\x00\x00\x00\x00\x80\x00\x00\x00\x00\x00\x8c\x00\x00\x01\x16\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\xff\xff\xff\xff\xbcL\xdb}\x10{\xe2w\xaa\xdb"rC\xdf\xb3\xe0M\xd5,\x81\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00x\x9c%\xcd\xb9\rB1\x10\x00Q\xc7\xbf\x19\xf6\xb6\xdd\x08\xb9\xf7\x92H\xa9\x90\xd2\xb8\x82\xc9\x9e4c\x8c\xfb\xf8\xf7\xca\xc7\x13n16\x8a\x88\xb2\xd8\x818`\xb4=eF\xb9f\x17\xcc\x92\x94hR\xc0\xeb\xe7s(/\x02\xcb\xd8\x13K\tU m\t\x1f\xef\xb2D\x03\xa6\xb6\x14\xb2\xaf\xc7[\rw?\x16`\xce\xd0"\x9c,\xddK\xd0c/\rIX4\xc3\xbc\xe4\xef{ u\xcc\x8c\x9c\x93]\x0f\x9cM;\n\xb7\x12-X\x1c\x96\x9fuT\xc8\xf5\x06\x88\xa25W\x00\x00\x00\x00\x01\x0c\x00\x00\x00\x00\x00<\x00\x00\x01\x16\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00\x02\xff\xff\xff\xff\x90#\x1d\xdc\xa3o\xa1x\xa0\xee\xd9\x9b\xd00x\x11$\x87\xdd\xa3\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xe6\x00\x00\x01\x16\x00\x00\x000dir1/f\x0028c776ae08d0d55eb40648b401b90ff54448348e\n',
    b''
  ]

Requesting both changelog and manifestlog works.

  $ sendhttpv2peer << EOF
  > command rawstorefiledata
  >     files eval:[b'changelog', b'manifestlog']
  > EOF
  creating http peer for wire protocol version 2
  sending rawstorefiledata command
  response: gen[
    {
      b'filecount': 2,
      b'totalsize': 1111
    },
    {
      b'location': b'store',
      b'path': b'00manifest.i',
      b'size': 584
    },
    b'\x00\x03\x00\x01\x00\x00\x00\x00\x00\x00\x00I\x00\x00\x00V\x00\x00\x00\x00\x00\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff\x99/Gy\x02\x9a=\xf8\xd0fm\x00\xbb\x92OicN&A\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00x\x9c\r\xca\xc1\x11\x00!\x08\x040\xdfV\x03+\xa2\x94\xb3\x8c\xd0\x7f\twy\x87\x03i\x95r\x96F6\xe5\x1c\x9a\x10-\x16\xba|\x07\xab\xe5\xd1\xf08s\\\x8d\xc2\xbeo)w\xa9\x8b;\xa2\xff\x95\x19\x02jB\xab\x0c\xea\xf3\x03\xcf\x1d\x16\t\x00\x00\x00\x00\x00I\x00\x00\x00\x00\x007\x00\x00\x00V\x00\x00\x00\x00\x00\x00\x00\x01\x00\x00\x00\x00\xff\xff\xff\xff\xa9\x88\xfbCX>\x87\x1d\x1e\xd5u\x0e\xe0t\xc6\xd8@\xbb\xbf\xc8\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00+\x00\x00\x00+a\x009a38122997b3ac97be2a9aa2e556838341fdf2cc\n\x00\x00\x00\x00\x00\x80\x00\x00\x00\x00\x00\x8c\x00\x00\x01\x16\x00\x00\x00\x01\x00\x00\x00\x02\x00\x00\x00\x01\xff\xff\xff\xff\xbcL\xdb}\x10{\xe2w\xaa\xdb"rC\xdf\xb3\xe0M\xd5,\x81\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00x\x9c%\xcd\xb9\rB1\x10\x00Q\xc7\xbf\x19\xf6\xb6\xdd\x08\xb9\xf7\x92H\xa9\x90\xd2\xb8\x82\xc9\x9e4c\x8c\xfb\xf8\xf7\xca\xc7\x13n16\x8a\x88\xb2\xd8\x818`\xb4=eF\xb9f\x17\xcc\x92\x94hR\xc0\xeb\xe7s(/\x02\xcb\xd8\x13K\tU m\t\x1f\xef\xb2D\x03\xa6\xb6\x14\xb2\xaf\xc7[\rw?\x16`\xce\xd0"\x9c,\xddK\xd0c/\rIX4\xc3\xbc\xe4\xef{ u\xcc\x8c\x9c\x93]\x0f\x9cM;\n\xb7\x12-X\x1c\x96\x9fuT\xc8\xf5\x06\x88\xa25W\x00\x00\x00\x00\x01\x0c\x00\x00\x00\x00\x00<\x00\x00\x01\x16\x00\x00\x00\x02\x00\x00\x00\x03\x00\x00\x00\x02\xff\xff\xff\xff\x90#\x1d\xdc\xa3o\xa1x\xa0\xee\xd9\x9b\xd00x\x11$\x87\xdd\xa3\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xe6\x00\x00\x01\x16\x00\x00\x000dir1/f\x0028c776ae08d0d55eb40648b401b90ff54448348e\n',
    b'',
    {
      b'location': b'store',
      b'path': b'00changelog.i',
      b'size': 527
    },
    b'\x00\x01\x00\x01\x00\x00\x00\x00\x00\x00\x00@\x00\x00\x00?\x00\x00\x00\x00\x00\x00\x00\x00\xff\xff\xff\xff\xff\xff\xff\xff3\x90\xef\x85\x00s\xfb\xc2\xf0\xdf\xff"D4,\x8e\x92)\x01:\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00u992f4779029a3df8d0666d00bb924f69634e2641\ntest\n0 0\na\nb\n\ncommit 0\x00\x00\x00\x00\x00@\x00\x00\x00\x00\x00>\x00\x00\x00=\x00\x00\x00\x01\x00\x00\x00\x01\x00\x00\x00\x00\xff\xff\xff\xffD2\xd86&\xe8\xa9\x86U\xf0b\xec\x1f*C\xb0\x7f\x7f\xbb\xb0\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00ua988fb43583e871d1ed5750ee074c6d840bbbfc8\ntest\n0 0\na\n\ncommit 1\x00\x00\x00\x00\x00~\x00\x00\x00\x00\x00N\x00\x00\x00W\x00\x00\x00\x02\x00\x00\x00\x02\x00\x00\x00\x01\xff\xff\xff\xff\xa4r\xd2\xea\x96U\x1a\x1e\xbb\x011-\xb2\xe6\xa7\x86\xd0F\x96o\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00x\x9c%\xc5\xc1\t\xc0 \x0c\x05\xd0{\xa6p\x03cjI\xd71\xf9\x11<H\xa1u\x7fJ\xf1]\x9eyu\x98\xa2\xb0Z\x88jk0\x11\x95z\xa0\xdb\x11\\\x81S\xfc*\xb4\xe2]\xc4\x89\t\xe3\xe1\xec;\xfc\x95\x1c\xbbN\xe4\xf7\x9cc%\xf9\x00S#\x19\x13\x00\x00\x00\x00\x00\xcc\x00\x00\x00\x00\x00C\x00\x00\x00B\x00\x00\x00\x03\x00\x00\x00\x03\x00\x00\x00\x02\xff\xff\xff\xff\x85kg{\x94a\x12i\xc5lW5[\x85\xf9\x95|\xfc\xc1\xb9\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00u90231ddca36fa178a0eed99bd03078112487dda3\ntest\n0 0\ndir1/f\n\ncommit 3',
    b''
  ]

  $ cat error.log
