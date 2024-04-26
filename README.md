# Rotto

Rotto是用Rust语言实现的一个用于生成符合“乐透”玩法的数列随机生成器，学习阶段，仅供娱乐。

## 模式
```
1. 大乐透
2. 双色球
```

## 生成方式
1. 随机生成
2. 用当前日期进行Sha512哈希运算，转为数字切片后顺序获取符合条件的数字

## 开发计划
+ [x] 记录运算日志
+ [ ] 接入往期记录，加入概率

