### 如何使用该校园网自动登录脚本
1. 克隆本仓库到本地

` git clone git@github.com:smallcjy/SCUT-Net-Auto-Login.git `

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
