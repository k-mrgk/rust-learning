## learning_8-1

整数のリストが与えられ、ベクタを使って mean(平均値)、median(ソートされた時に真ん中に来る値)、 mode(最も頻繁に出現する値; ハッシュマップがここでは有効活用できるでしょう)を返してください。

```
$ cargo run
Input list of integers(e.g. 1 2 3 4 ) >  1 1 1 2 2 3 4
mean value is 2.
median value is 2.
mode value is 1.

$ cargo run
Input list of integers(e.g. 1 2 3 4 ) > 10 20 30 40 50
mean value is 30.
median value is 30.
mode value is 30. // 最頻値が複数ある場合はどれかの値が出力される

```
