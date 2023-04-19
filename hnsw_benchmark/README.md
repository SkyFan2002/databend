## instant_distance
crate地址：https://crates.io/crates/instant-distance
测试环境：福州开发机
测试数据：一百万条随机生成的三维向量，约20M
测试代码：test_1.rs
生成HNSW索引用时：12.904159393s
索引序列化（Json）之后的体积：545M
使用HNSW计算10个最近邻的向量，总用时：171.609µs
使用暴力搜索，计算10个最近邻的向量，总用时：2.304406ms
使用HNSW的召回率：100%

