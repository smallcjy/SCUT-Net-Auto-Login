### 如何使用该校园网自动登录脚本

**用前须知：**

- 由于工具开发不尽完善，该工具的使用场景为在宿舍环境下，校园网具备自动连接的习惯且优先连接校园网的情况下，才可做到自动登录。
- 由于作者的开发测试环境是大学城校区，五山校区不保证能正常运行
- 只可在开机时自动连接，如果后续有断开的情况，无法自动连接。
- 实现这些需求是未来的开发计划，请谅解。
- 请仔细阅读本文档！
的情况下，
1. 克隆本仓库到本地或者直接下载压缩包本体

` git clone git@github.com:smallcjy/SCUT-Net-Auto-Login.git `

或者点击右上角的code，点击local，点击获取压缩包下载，解压。

2. 打开克隆仓库文件的位置，打开_internal文件夹，找到useraccount.txt和password.txt分别写入校园网登录账号和密码，保存退出。
3. 设置开机自启
  - 返回本地的仓库地址，右键autoLogin.exe创建快捷方式
  - 按下win+R，输入`shell:startup`，点击确定
  - 把上面创建的快捷方式拖入到出现的文件夹中

### 如何取消开机自启
  - 按下win+R，输入`shell:startup`，点击确定
  - 在出现的文件夹中把快捷方式删除

**注意：开机自启后会一闪而过一个控制台，如果介意影响美观，则不要使用！**

**注意：本脚本仅供SCUT的同学在宿舍环境下使用！**

**注意：本工具完全免费，如果遇到售卖的情况，请发issue告知作者，作者会立刻开源源代码！**

**注意：先在开机时运行exe测试一下，成功再设置开机自启。**

**如果该工具给你带来便利的体验，请点一颗免费的star！这是支持作者继续完善工具的最大动力！**
