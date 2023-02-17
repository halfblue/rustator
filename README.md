# rustator

用于生成子域名字典，参考
altdns
dnsgen
gotator
等工具，基本覆盖上述工具的规则

### 安装
``` bash
git clone https://github.com/halfblue/rustator.git
Cargo build --release
```

### 运行
```bash
./target/release/rustator -h
./target/release/rustator -d domains.txt -w words.txt
```
domains.txt是已收集的子域名，words.txt是用于生成的词典
默认不需要改参数，如果子域名数量较大(>2000)，建议调整参数或词典，否则最终生成字典可能过大(>1G)

### 验证
使用shuffledns、ksubdomain等dns验证工具即可
