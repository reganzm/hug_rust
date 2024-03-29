**开篇词**

**要干嘛**：写一个Rust系列

**啥目的**：带大家学习未来语言<font color="red">Rust</font> :smile:

**系列叫啥名字**：拥抱未来语言Rust

**发文时间**：每周六和每周日:laughing:

**预计章节**：60回

**互动方式**：微信群和公众号(文末有加入方式)

##### 整个系列布局：
- 第一部分：Rust语法快速学习
- 第二部分：Rust设计模式
- 第三部分：Rust数据结构和算法，爽刷Leecode
- 第四部分：Rust精湛小项目，含web开发、量化开发
  

![系列布局](./images/系列布局v2.png)
---

整个系列需要接收读者的反馈和答疑，以便更好的修正文章内容，所以建立微信群增加互动性，你可以在微信群里面提出反馈意见，或者和群友进行心得交流，亦或进行催更呐喊。加入方式是加我微信好友，备注：rust-昵称-其它信息例如：rust-蔓蔓学-大数据，我会将备注为本格式的好友邀请至微信群。

![微信二维码](./images/微信二维码.png)

---

不出意外的话，以后每个周末都会更新一到两篇文章哦，不过文章还是以质量和准确性为主，不会带着问题强行更新的。

再次声明，本系列<font color='red'>完全免费</font>，直到所有章节结束，所以你们的喜爱和传播就是我坚持更新的最大动力，为了不错过更新，可以星标我的公众号防止错过更新:smile:。最后，希望大家喜欢这个系列，可以多多帮忙传播，例如：朋友圈打卡、点个再看，或者你也写博客的话可以在文章中提提我:smile:，在此多谢各位捧场！

本系列文章，我也会在GitHub上进行同步，因为公众号发送之后修改很麻烦，且有一定的修改次数的限制，没办法进行整体的调整。感兴趣的可以点击阅读全文进入GitHub，同时所有的源代码也在GitHub上哦，感兴趣的可以star一下:smile:。
让我们一起期待吧。


### 微信公众号文章传送门
[开篇词](https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484162&idx=1&sn=c2b12585654d3231775b13d14fbbcf0f&chksm=cfe11d30f8969426f5f94e74ffe33b273a52daef7ebf65234155a8343d7c82ba127cf68705bf&token=717589962&lang=zh_CN#rd)

[番外篇 我的成长故事](https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484168&idx=1&sn=4a5c2cebc7e958d038288dd231f56048&chksm=cfe11d3af896942c9638bb12463c2faa94a57f0c14676bf977483df46ba3aa072a519cee380a&token=717589962&lang=zh_CN#rd)

### 基础篇 

[第一回 环境准备](https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484170&idx=1&sn=4e880a4ca55af9dfa489469ba6b02370&chksm=cfe11d38f896942ee5f086ac08949e69604df2a71701bba80c4e2bcb88e3d45b444562f6bbb4&token=717589962&lang=zh_CN#rd)

[第二回 第一性原理看类型系统](https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484186&idx=1&sn=201e3b84de4c813844cc87bddf13a2fd&chksm=cfe11d28f896943e6df94027e6ad50acae6646cddb85150783b15baa76c76c1498ca534c6c29&token=717589962&lang=zh_CN#rd)

[第三回 基本约定](https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484197&idx=1&sn=3e1ff57800c00bae425a97a77cdec0d6&chksm=cfe11d17f8969401a39414536420aee96a30e18f011b13fd03f2d9fb1266520c08c595129065&token=2086026546&lang=zh_CN#rd)

[第四回 认识变量常量和标量](https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484205&idx=1&sn=ec2e875d1b3930cf4d3dc7ba5c00a79f&chksm=cfe11d1ff8969409c39fd79c4d9e2f82033e6f43a55924cab9b474552647816963ea12f8298d&token=2086026546&lang=zh_CN#rd)

[第五回 Rust中的核心数据类型](https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484211&idx=1&sn=76a20ae3dc77827a046914e61edd28d7&chksm=cfe11d01f89694172777713897dd36fd5b3a639289e0c816a51d2ebb8a1b792c1c3753252539&token=2086026546&lang=zh_CN#rd)

[第六回 Rust泛型](https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484216&idx=1&sn=31465680eb95faa7b4631eaef1e2e504&chksm=cfe11d0af896941c2cf414fcb141ae8ffe7eb24f312731d62934f6b1fa9351939b53b5aec63b&token=2086026546&lang=zh_CN#rd)

[第七回 Rust的灵魂特征](https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484225&idx=1&sn=8981c62dd4e2cb9b3d4ec4d65e70b1c9&chksm=cfe11d73f89694652a7566584e4ba878070eb0695f9afde71280c9221883abf110620549e69a&token=2086026546&lang=zh_CN#rd)

[第八回 所有权和引用](https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484234&idx=1&sn=bb345b5beb1ed7d683f604b21d5eedcd&chksm=cfe11d78f896946eefc8d5bcd46e07ad831a17b5acea4ba8abd7b6d09981727ee8b8da9cfd84&token=1876812958&lang=zh_CN#rd)

[第九回 生命周期](https://mp.weixin.qq.com/s?__biz=Mzg5MDE5NDc4MQ==&mid=2247484240&idx=1&sn=9974e08e82d2ffa3e4e9006588a6c897&chksm=cfe11d62f8969474a66050ea3683eff82bb06cd18ef4a46a07862cbed485d6a8b4e9420d435e&token=1876812958&lang=zh_CN#rd)

##### 添加公众号
![微信公众号](./images/wechat_service.jpg)