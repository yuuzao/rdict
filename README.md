

# Rdict
一个命令行词典工具。
![](https://s2.loli.net/2022/03/27/k7NlbLTQrVwZ5zn.png)

## Usage
```
>> rdict -h
USAGE:
    rdict [OPTIONS]

OPTIONS:
    -d, --dict <DICT>           Where do you want to query from? [default: youdao] [possible values:
                                youdao, bing]
    -h, --help                  Print help information
    -l, --list <LIST>           list query history
    -p, --phrase <PHRASE>...    What do you want to query?
    -v, --voice <VOICE>         query with voice, uk or 1 for uk, us or 2 for us [possible values:
                                us, uk, 1, 2]
    -V, --version               Print version information

```
![usage.gif](https://s2.loli.net/2022/03/27/T814YpBElubOfs2.gif)

### 目前支持功能：
目前只支持`Linux`系统。
1. 查词
   ```
   rdict -p hello
   ```
   目前只支持从有道查询，Bing暂时不支持，以后可能支持本地、维基百科或者其他来源。
2. 语音
   ```
   rdict hello -v
   ```
   可以指定英美发音
3. 查看N条查询历史
    ```
    rdict -l 123
    ```
    默认显示5条历史

### TODO

1. 导出生词本到有道或者Anki
2. 显示更多例句
3. 更多的查询来源
4. 跨平台