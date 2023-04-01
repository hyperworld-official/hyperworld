# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.14.0] - 2023-01-07

### Added

- Setting for disabling flashing lights
- Spectate mode for moderators.
- Currently playing music track and artist now shows in the debug menu.
- Added a setting to influence the gap between music track plays.
- Added a Craft All button.
- Server: Vacuum database on startup
- SeaChapel, greek/latin inspired dungeon for ocean biome coasts
- Entity view distance setting added (shown in graphics and network tabs). This setting controls
  the distance at which entities are synced to the client and which entities are displayed in.
  This is clamped to be no more than the current overall view distance setting.
- View distance settings that are lowered by the server limit (or other factors) now display an
  extra ghost slider cursor when set above the limit (instead of snapping back to the limit).
  Limits on the view distance by the server no longer affect the settings saved on the client.
- HQX upscaling shader for people playing on low internal resolutions
- Pets can now be traded with.
- Crafting recipe for black lantern
- Added redwood and dead trees
- Water will now move according to its apparent flow direction
- Added screen-space reflection and refraction shaders
- Added reflection quality setting
- UI: Added a poise indicator to the player's status bars
- FxUpscale AA mode for higher quality graphics at reduced internal resolutions
- Graphics presets
- Sword
- Doors now animate opening when entities are near them.
- Musical instruments can now be crafted, looted and played
- NPCs now move to their target's last known position.
- Experience bar below the hotbar
- Bridges.
- Tool for exporting PNG images of all in-game models (`cargo img-export`)
- Calendar event soundtracks.
- 禁用闪光灯的设置
- 主持人的观察模式。
- 当前播放的音乐曲目和艺术家现在显示在调试菜单中。
- 增加了影响音乐曲目播放间隔的设置。
- 增加了一个 "全部制作 "按钮。
- 服务器。启动时真空数据库
- SeaChapel，受希腊/拉丁语启发的海洋生物群落海岸的地牢。
- 增加了实体视图距离设置（显示在图形和网络标签中）。这个设置控制
  实体同步到客户端的距离以及实体显示的距离。
  这被钳制为不超过当前的整体视图距离设置。
- 被服务器限制（或其他因素）降低的视图距离设置现在显示一个
  额外的幽灵滑块光标（而不是缩回到极限）。
  服务器对视距的限制不再影响保存在客户端的设置。
- 为在低内部分辨率下游戏的人提供了HQX升频着色器。
- 现在可以用宠物进行交易了。
- 黑灯笼的制作配方
- 增加了红木和枯树
- 水现在将根据其明显的流动方向移动
- 添加了屏幕空间的反射和折射着色器
- 添加了反射质量设置
- UI。在玩家的状态栏中添加了一个姿态指示器
- FxUpscale AA模式用于在降低内部分辨率时获得更高质量的图形
- 图形预置
- 剑
- 当实体靠近门时，门现在可以动画化打开。
- 乐器现在可以被制作、掠夺和演奏了
- NPC现在可以移动到其目标的最后位置。
- 热量条下方的经验条
- 桥梁。
- 用于导出所有游戏内模型的PNG图像的工具（`cargo img-export`）。
- 日历事件配乐。

### Changed

- Use fluent for translations
- First tab on Login screen triggers username focus
- Certain NPCs will now attack when alone with victim
- /kill_npcs no longer leaves drops behind and also has bug causing it to not destroy entities
  fixed.
- Default present mode changed to Fifo (aka 'Vsync capped').
- Old "Entity View Distance" setting renamed to "Entity Detail Distance" (since this controls the
  distance at which lower detail models are used for entities).
- Present mode options renamed for clarity: Fifo -> 'Vsync capped', Mailbox -> 'Vsync uncapped',
  Immediate -> 'Vsync off'.
- Item pickup UI now displays items that members of your group pick up.
- Improved shiny water shaders
- Tweaked armor stats
- Move bag icon to skillbar
- Improved inventory sorting by Category
- 使用流利说进行翻译
- 登录屏幕上的第一个标签会触发用户名焦点
- 某些NPC现在会在与受害者单独相处时进行攻击
- /kill_npcs不再留下空投，也有了导致它不能摧毁实体的错误。
  修复了。
- 默认的现在模式改为Fifo（又称 "Vsync capped"）。
- 旧的 "实体视图距离 "设置更名为 "实体细节距离"（因为它控制了使用低细节模型的距离）。
  因为它控制了低细节模型用于实体的距离）。
- 为清晰起见，现在的模式选项重新命名。Fifo -> "Vsync capped", Mailbox -> "Vsync uncapped"。
  立即 -> "Vsync关闭"。
- 物品拾取界面现在可以显示你的小组成员拾取的物品。
- 改进了闪亮的水的着色器
- 调整了盔甲的统计数据
- 将包的图标移至技能栏
- 改进了按类别进行的库存排序

### Removed

### Fixed

- Fixed npc not handling interactions while fighting (especially merchants in trade)
- Fixed bug where you would still be burning after dying in lava.
- Workaround for rayon bug that caused lag spikes in slowjobs
- Fixed crash due to zooming out very far
- Client properly knows trade was cancelled when exiting to the character screen (and no longer
  tries to display the trade window when rejoining)
- Cancel trades for an entity when it is deleted (note this doesn't effect trades between players
  since their entities are not removed).
- Fixed bug where the view distance selection was not immediately applied to entity syncing when
  first joining a server and when changing the view distance (previously this required moving to a
  new chunk for the initial setting or subsequent change to apply).
- Moderators and admins are no longer blocked from logging in when there are too many players.
- FXAA now behaves correctly at non-1.0x internal resolutions
- Pets no longer aggro on pet owners after being healed
- Pets no longer lose their intrinsic weapons/armour when loaded on login.
- Fixed npcs using `/say` instead of `/tell`
- Camera jittering in third person has been significantly reduced
- Many water shader issues have been fixed
- Flee if attacked even if attacker is not close.
- `/time` command will never rewind time, only advance it to not break rtsim
- 修正了npc在战斗时不能处理互动的问题（特别是贸易中的商人）。
- 修正了在熔岩中死亡后仍会燃烧的错误。
- 解决了在慢动作中导致滞后尖峰的雷恩错误。
- 修正了由于放大了很远而导致的崩溃
- 当退出到角色屏幕时，客户端正确地知道交易被取消了（并且不再
  试图在重新加入时显示交易窗口)
- 当一个实体被删除时，取消它的交易（注意这不会影响玩家之间的交易
  因为他们的实体没有被删除）。
- 修正了一个错误，即在第一次加入服务器时，查看距离的选择没有立即应用到实体同步中。
  时，不会立即应用于实体同步（以前这需要移动到一个新的区块进行初始设置或改变视图距离）。
  新的区块来应用最初的设置或随后的改变）。
- 当有太多的玩家时，版主和管理员不再被阻止登录。
- FXAA现在在非1.0x内部分辨率下表现正确。
- 宠物在被治疗后不再对宠物主人进行攻击
- 宠物在登录时不再失去其固有的武器/盔甲。
- 修复了npc使用`/say`而不是`/tell`的问题。
- 第三人称摄影机的抖动已大大减少。
- 许多水的着色器问题已被修复
- 即使攻击者不在附近，如果被攻击也会逃跑。
- `/time`命令永远不会让时间倒退，只让它提前，以免破坏rtsim。

## [0.13.0] - 2022-07-23

### Added

- Chat commands to mute and unmute players
- Waypoints saved between sessions and shared with group members.
- New rocks
- Weapon trails
- Hostile agent will now abort pursuing their target based on multiple metrics
- Admin command to reload all chunks on the server
- Furniture and waypoints in site2 towns
- Text input for trading
- Themed Site CliffTown, hoodoo/arabic inspired stone structures inhabited by mountaineer NPCs.
- NPCs now have rudimentary personalities
- Added Belarusian translation
- Add FOV check for agents scanning for targets they are hostile to
- Implemented an LoD system for objects, making trees visible far beyond the view distance
- Add stealth stat on Bag UI
- Water caves
- Modular weapons
- Added Thai translation
- Skiing and ice skating
- Added loot ownership for NPC drops
- Bamboo collectibles now spawn near rivers
- Chest sprites can longer be exploded
- Smoke varies by temperature, humidity, time of day and house
- Added loot ownership for drops from mining
- Added an option for experience number accumulation.
- Added an option for damage number rounding (when greater than or equal to 1.0).
- Added sliders for incoming/non-incoming damage accumulation duration.
- New ambience sounds
- Slider for ambience volume
- Weather generated on server is sent to clients, and seen on clients as rain/clouds.
- Updated Brazilian Portuguese Translation
- Lightning storms
- More varied ambient birdcalls
- Cave biomes
- Updated the Polish translation
- 聊天指令可使玩家静音和取消静音
- 会话之间保存的航点，并与小组成员共享。
- 新的岩石
- 武器追踪
- 敌方特工现在会根据多个指标放弃追击他们的目标
- 管理员命令重新加载服务器上的所有块状物
- 在site2城镇的家具和航点
- 交易的文本输入
- 主题网站CliffTown，由登山者NPC居住的hoodoo/arabic灵感石头结构。
- NPC现在有了基本的个性
- 增加了白俄罗斯语翻译
- 为扫描敌方目标的特工增加FOV检查。
- 为物体实现了LoD系统，使树木在视距之外也能看到。
- 在Bag UI上添加隐身状态
- 水洞
- 模块化武器
- 添加泰语翻译
- 滑雪和滑冰
- 增加了NPC掉落的战利品所有权
- 竹子收藏品现在在河流附近产生
- 胸部精灵不再能被爆炸了
- 烟雾因温度、湿度、时间和房屋而异
- 增加了采矿掉落的战利品所有权
- 增加了经验值累积的选项。
- 增加了伤害数字取整的选项（当大于或等于1.0时）。
- 增加了来袭/非来袭伤害累积时间的滑块。
- 新的环境声音
- 氛围音量的滑块
- 在服务器上生成的天气被发送到客户端，并在客户端看到雨/云。
- 更新了巴西葡萄牙语翻译
- 闪电风暴
- 更多样的环境鸟叫声
- 洞穴生物群落
- 更新了波兰语翻译

### Changed

- Improved site placement
- [Server] Kick clients who send messages on the wrong stream
- Reworked Merchant trade price calculation, Merchants offer more wares
- Enable new giant trees, changed what entities spawn at them
- Stealth is now shown as a percentage in Stats Diary UI
- Stealth effects from sneaking and armor are evaluated independently. Armor now has effects even when not sneaking
- Zoom-in effect when aiming bow is now optional
- Non-Humanoid NPCs now pick up consumables when less than full health and use them to heal up.
- Changed module component modifier costs to the following scheme, based on base material: 1 -> 2 -> 5 -> 10 -> 15 -> 25
- Damage from the same source dealt in the same tick will now be grouped up.
- Critical hits are now shown differently in the damage numbers.
- Fall damage and some (extra) buffs/debuffs now show up in the damage numbers.
- Optimized sprite processing decreasing the startup time of voxygen (and long freezes when trying
  to enter the world when this hasn't finished).
- Metadata added to music files. Listen to the soundtrack more easily!
- Overhauled caves: they're now a multi-layer network spanning the entire world
- 改进网站布局
- 服务器] 踢掉在错误信息流上发送信息的客户
- 重整了商人的贸易价格计算，商人提供了更多的商品。
- 启用新的巨树，改变了在巨树上产生的实体。
- 隐身术现在以百分比的形式显示在统计日记的用户界面上。
- 潜行和盔甲的隐身效果被独立评估。盔甲现在即使在不潜行的情况下也有效果。
- 瞄准弓箭时的放大效果现在是可选的。
- 非人形NPC现在可以在健康状况不佳时拾取消耗品，并使用它们来进行治疗。
- 根据基本材料，将模块组件的修改费用改为以下方案。1 -> 2 -> 5 -> 10 -> 15 -> 25
- 来自同一来源的伤害在同一时间段内会被分组。
- 关键打击现在以不同的方式显示在伤害数字中。
- 坠落伤害和一些（额外的）BUFF/DEBUFF现在显示在伤害数字中。
- 优化了精灵的处理，减少了voxygen的启动时间（以及在尝试进入世界时的长时间冻结）。
  的启动时间（以及在这一过程尚未完成时试图进入世界的长时间冻结）。
- 为音乐文件添加了元数据。听原声带更容易了!
- 大修了洞穴：它们现在是一个横跨整个世界的多层网络。

### Removed

- Removed the options for single and cumulated damage.
- 删除了单次和累积伤害的选项。

### Fixed

- Fixed bug that would sometimes cause taking a screenshot to panic because a buffer was mapped at the wrong time.
- Players can no longer push waypoints around
- Sites will now also be placed near the edge of the map
- Fix a bug causing NPCs to jitter on interaction and randomly run away.
- Harvester boss arenas should be more accessible and easier to exit
- Fix agents not idling
- Fixed an error where '{amount} Exp' floater did not use existing localizations
- Fix villagers seeing cultists and familiar enemies through objects.
- Menacing agents are now less spammy with their menacing messages
- Fixed the title screen FPS cap not applying when the background FPS limit was set higher than 60 FPS
- Fixed an issue where the hurt animation would "jump" whenever you lost/gained health.
- Fixed a bug where multiple damage sources in the same tick would show up as a singular attack.
- Fixed an issue where, if the same amount of healing and damage was received in the same tick, nothing would be shown.
- UI sfx now play from UI instead of from camera (allowing stereo sfx)
- Most sfx now correctly play when camera is underwater
- All sounds now stop upon quitting to main menu
- Combat music now loops and ends properly
- Modular weapons now have a selling price
- Closing a subwindow now only regrabs the cursor if no other subwindow requires it.
- 修正了有时会因为缓冲区在错误的时间被映射而导致拍摄截图时出现恐慌的错误。
- 玩家不能再将航点推来推去了
- 站点现在也会被放置在地图的边缘附近
- 修复了一个导致NPC在互动时抖动并随机跑开的错误。
- 收割机老板竞技场应该更容易进入，更容易退出
- 修复代理不空转的问题
- 修正了一个错误，即"{amount}Exp "的浮子没有使用现有 Exp'的浮动器没有使用现有的本地化。
- 修复村民通过物体看到邪教分子和熟悉的敌人的问题。
- 来势汹汹的特工现在没有那么多垃圾信息了
- 修正了当背景FPS限制设置为高于60FPS时，标题屏幕FPS上限不适用的问题
- 修正了一个问题，即当你失去/获得健康时，伤害动画会 "跳跃"。
- 修正了一个错误，即在同一个勾股中的多个伤害源会显示为一个单一的攻击。
- 修正了一个问题，即如果在同一时间内收到相同数量的治疗和伤害，则不会显示任何东西。
- UI音效现在从UI播放，而不是从摄像机播放（允许立体声音效）。
- 大多数音效现在可以在相机处于水下时正确播放。
- 所有声音现在都会在退出主菜单时停止。
- 战斗音乐现在可以正确地循环播放并结束
- 模块化武器现在有了销售价格
- 关闭子窗口现在只在其他子窗口不需要的情况下才会重新捕获光标。

## [0.12.0] - 2022-02-19

### Added

- Added a setting to always show health and energy bars
- Added a crafting station icon to the crafting menu sidebar for items that could be crafted at a crafting station
- Added a setting to disable the hotkey hints
- Added a credits screen in the main menu which shows attributions for assets
- Shrubs, a system for spawning smaller tree-like plants into the world.
- Waterfalls
- Sailing boat (currently requires spawning in)
- Added a filter search function for crafting menu, use "input:______" to search for recipe inputs
- Added catalan (Catalonia) language translation
- Sneaking with weapons drawn
- Stealth stat values on (some) armors
- All new dismantling interface found at your nearest dismantling staion
- Wearable headgear, including hood, crown, bandanas
- Bomb sprites (can be exploded with arrows or other explosions)
- Campfire waypoints in towns
- Arbitrary volume entities
- New outfit for merchants
- Nightly linux Aarch64 builds are now produced (distribution via airshipper will follow soon)
- Worldgen wildlife density modifier in features.ron
- Rivers now make ambient sounds (again)
- Added a setting to see own speech bubbles
- Added an option to allow players to remove keybindings
- Piercing damage now ignores an amount of protection equal to damage value
- Slashing damage now reduces target's energy by an amount equal to damage dealt to target post-mitigation
- Crushing damage now does poise damage to a target equal to the amount mitigated by armor
- UI to select abilities and assign to hotbar
- Position of abilities on hotbar is now persisted through the server
- Interation hints now appear for sprites and entities
- Players can now mount and ride pets
- Experimental shaders, that can be enabled in Voxygen's settings (see the book for more information)
- Keybinding customization to set waypoint on Map
- Added arthropods
- A 'point light glow' effect, making lanterns and other point lights more visually pronounced
- Generate random name for site2 sites
- Shader dithering to remove banding from scenes with large colour gradients
- Convert giant trees to site2
- Add new upgraded travelers
- Wallrunning
增加了一个设置，可以始终显示健康和能量条
- 在制作菜单侧边栏中为可以在制作站制作的物品添加了一个制作站的图标
- 增加了禁用热键提示的设置
- 在主菜单中增加了一个显示资产属性的功劳屏幕
- 灌木，一个用于在世界中生成较小的树状植物的系统。
- 瀑布
- 帆船（目前需要产卵进入）。
- 为手工制作菜单添加了过滤搜索功能，使用 "input:______"来搜索配方的输入。
- 添加了加泰罗尼亚语（Catalonia）语言翻译
- 拔出武器时的潜行
- 在（某些）盔甲上的隐身状态值
- 在你最近的拆解站找到所有新的拆解界面
- 可佩戴的头饰，包括头巾、皇冠、手帕等
- 炸弹精灵（可以用箭或其他爆炸物进行爆炸）
- 城镇中的篝火路标
- 任意体积的实体
- 商人的新装束
- 现在已经制作了夜间Linux Aarch64版本（通过airshipper发布的版本很快就会出现）。
- 在features.ron中添加了Worldgen野生动物密度修改器
- 河流现在可以发出环境音了（再次）。
- 增加了一个设置，可以看到自己的语音气泡
- 增加了允许玩家删除键盘绑定的选项
- 穿刺伤害现在无视与伤害值相等的保护量
- 割裂伤害现在使目标的能量减少，减少的量等于对目标造成的伤害后的缓解。
- 碾压伤害现在对目标造成的姿态伤害等于护甲的缓解量
- 选择能力和分配到热键的用户界面
- 热线条上的能力位置现在可以通过服务器持续保持。
- 对精灵和实体的干预提示现在出现了。
- 玩家现在可以骑乘宠物了
- 实验性着色器，可以在Voxygen的设置中启用（更多信息见本书）
- 自定义键位以在地图上设置航点
- 增加了节肢动物
- 一个 "点灯发光 "的效果，使灯笼和其他点灯在视觉上更明显。
- 为site2站点生成随机名称
- 着色器抖动，以消除具有大色彩梯度的场景中的带状现象
- 将巨树转换为site2
- 添加新的升级版旅行者
- 墙面跑动

### Changed

- Made dungeon tiers 3, 4, and 5 more common
- Put date at the begining of the log file instead of the end to allow MIME type recognition
- Tweaked CR and exp calculation formula
- Sprite spawn rates
- The Interact button can be used on campfires to sit
- Made map icons fade out when near the edge of the map display
- Roughly doubled the speed of entity vs terrain physics checks
- Updated client facing error messages to be localizable strings
- Nerfed some skill values
- Tweaked critical chance of legendary weapons
- Agents using fireball projectiles aim at the feet instead of the eyes
- Explosions can now have a nonzero minimum falloff
- EXP on kill is now shared based on damage contribution
- Dungeons have somewhat proper scaling. The higher the dungeon the harder it gets, Cultist staying unchanged while Mino is now at its level.
- Parallelized entity sync system on the server
- Item color backgrounds are now lighter
- All items that used the PNG file format now have a VOX file instead
- Yeti loot table modified
- Phoenix feathers are now Legendary quality
- Green/Red lantern now shine their respective color instead of the default lantern color
- Poise damage dealt to a target that is in a stunned state is now converted to health damage at an efficiency dependent on the severity of the stunned state
- You are now immune to poise damage for 1 second after leaving a stunned state
- Removed or reduced poise damage from most abilities
- Made the hotbar link to items by item definition id and component composition instead of specific inventory slots.
- Made loot boxes drop items instead of doing nothing in order to loot forcing
- Refactored agent code file structure
- Changed the way light strength is rendered by moving processing from shader code (GPU) to CPU code
- Bumped tracing-subscriber to resolve [RUSTSEC-2022-0006](https://rustsec.org/advisories/RUSTSEC-2022-0006)
- Made /home command a mod+ exclusive
- Friendly creatures will now defend each other
- Creatures will now defend their pets
- [WorldGen] Change path colors
- Render item drops instead of placeholder textures
- Arthropods are rebalanced
- Slight hat item rebalance (hats are more specialized and befitting of their rarity rank)
- Harvester boss buffed in stats
- 让地牢的第3、4、5层更加普遍。
- 将日期放在日志文件的开头，而不是结尾，以便于识别MIME类型。
- 调整了CR和Exp的计算公式
- 精灵产卵率
- 在篝火上可以使用互动按钮来坐着。
- 让地图图标在接近地图显示的边缘时淡出。
- 大致提高了实体与地形的物理学检查的速度
- 更新了客户端的错误信息，使之成为可本地化的字符串
- 削弱了一些技能值
- 调整了传奇武器的关键几率
- 使用火球弹的特工瞄准的是脚而不是眼睛
- 爆炸现在可以有一个非零的最小回落值
- 杀人时的经验值现在是根据伤害贡献来分享的。
- 地牢有了一些适当的比例。地牢越高，难度越大，崇拜者保持不变，而米诺现在处于其水平。
- 在服务器上并行了实体同步系统
- 物品的颜色背景现在变浅了
- 所有使用PNG文件格式的物品现在都用VOX文件代替。
- 修改了雪人战利品表
- 凤凰的羽毛现在是传奇品质
- 绿/红灯笼现在可以照亮它们各自的颜色，而不是默认的灯笼颜色。
- 对处于眩晕状态的目标造成的姿态伤害现在可以转换为健康伤害，其效率取决于眩晕状态的严重程度。
- 你现在可以在离开眩晕状态后的1秒内免于受到姿态伤害。
- 从大多数能力中移除或减少了毒气伤害
- 使得热键按物品定义ID和部件组成而不是特定的库存槽链接到物品。
- 让战利品箱掉落物品，而不是什么都不做，以便强行掠夺。
- 重构了代理人代码文件结构
- 通过将处理从着色器代码（GPU）转移到CPU代码，改变了光线强度的渲染方式
- 颠覆了追踪-用户，以解决[RUSTSEC-2022-0006](https://rustsec.org/advisories/RUSTSEC-2022-0006)
- 使得/home命令成为mod+的专属命令
- 友好的生物现在会互相保护
- 生物现在会保护他们的宠物
- [WorldGen] 改变路径颜色
- 渲染物品掉落而不是占位符纹理
- 节肢动物被重新平衡
- 轻微的帽子物品重新平衡（帽子更专业，更符合它们的稀有等级）。
- 收割机老板的状态得到增强

### Removed

- Removed unused PNG files
- Removed bomb_pile

### Fixed

- The menu map now properly handles dragging the map, zooming, and setting the waypoint when hovering icons
- Falling through an airship in flight should no longer be possible (although many issues with airship physics remain)
- Avoided black hexagons when bloom is enabled by suppressing NaN/Inf pixels during the first bloom blur pass
- Many know water generation problems
- Trading over long distances using ghost characters or client-side exploits is no longer possible
- Merchant cost percentages displayed as floored, whole numbers
- Bodies of water no longer contain black chunks on the voxel minimap.
- Agents can flee once again, and more appropriately
- Items in hotbar no longer change when sorting inventory
- Lantern color changes when swapping lanterns
- NPCs no longer wander off cliffs
- Guards will defend villagers instead of simply threatening the attacker
- Seafaring ships no longer spawn on dry land
- 菜单地图现在可以正确处理拖动地图、缩放和悬停图标时设置航点的问题。
- 飞行中的飞艇不再可能坠落（尽管飞艇物理学的许多问题仍然存在）。
- 通过在第一次绽放模糊过程中抑制NaN/Inf像素，避免在启用绽放时出现黑色的六边形。
- 许多已知的水生成问题
- 使用幽灵角色或客户端漏洞进行长距离交易已不再可能。
- 商人成本百分比显示为浮动的整数
- 水体在体素小地图上不再包含黑色的块状物。
- 代理人可以再次逃离，而且更合适。
- 在对库存进行分类时，热点栏中的物品不再改变。
- 交换灯笼时灯笼的颜色会改变
- NPC不再在悬崖上徘徊
- 警卫会保护村民而不是简单地威胁攻击者
- 海上航行的船只不再在陆地上产卵

## [0.11.0] - 2021-09-11

### Added

- Added a skill tree for mining, which gains xp from mining ores and gems.
- Added debug line info to release builds, enhancing the usefulness of panic backtraces
- NPCs and animals can now make sounds in response to certain events
- Players can press H to greet others
- Ability to toggle chat visibility
- Added gem rings with various stat improvements.
- Animations for using consumables.
- New danari character customizations
- Bald hairstyles for humans and danari
- AI for sceptre wielders and sceptre cultists in Tier 5 dungeons
- HUD debug info now displays current biome and site
- Quotes and escape codes can be used in command arguments
- Toggle chat with a shortcut (default is F5)
- Pets are now saved on logout 🐕 🦎 🐼
- Dualwielded, one-handed swords as starting weapons (Will be replaced by daggers in the future!)
- Healing sceptre crafting recipe
- NPCs can now warn players before engaging in combat
- Custom error message when a supported graphics backend can not be found
- Add server setting with PvE/PvP switch
- Can now tilt glider while only wielding it
- Experimental terrain persistence (see server documentation)
- Add GPU filtering using WGPU_ADAPTER environment variable
- Explosions no longer change block colours within safe zones
- The 'spot' system, which generates smaller site-like structures and scenarios
- Chestnut and cedar tree varieties
- Shooting sprites, such as apples and hives, can knock them out of trees
- Sprite pickup animations
- Add VELOREN_ASSETS_OVERRIDE variable for specifying folder to partially override assets.
- Cultist raiders
- Bloom Slider
- 增加了采矿的技能树，通过开采矿石和宝石获得经验。
- 在发布版本中增加了调试行信息，提高了恐慌回溯的有用性。
- NPC和动物现在可以对某些事件发出声音了。
- 玩家可以按H键来问候他人
- 能够切换聊天的可见性
- 增加了具有各种状态改进的宝石戒指。
- 使用消耗品的动画。
- 新的Danari角色定制
- 人类和Danari的秃头发型
- 5级地牢中的权杖挥舞者和权杖崇拜者的AI
- HUD调试信息现在可以显示当前的生物群落和地点。
- 引号和转义代码可以在命令参数中使用
- 用快捷键切换聊天（默认为F5）。
- 宠物现在在注销时被保存 🐕 🦎 🐼
- 双持单手剑作为起始武器（将来会被匕首取代！）。
- 愈合权杖的制作配方
- NPC现在可以在交战前警告玩家
- 找不到支持的图形后端时的自定义错误信息
- 增加带有PvE/PvP开关的服务器设置
- 现在可以在挥舞滑翔机时倾斜它了
- 实验性的地形持久性（见服务器文档）。
- 使用WGPU_ADAPTER环境变量增加GPU过滤功能
- 爆炸不再改变安全区的区块颜色
- 斑点 "系统，可以生成较小的类似场地的结构和场景
- 栗子树和雪松树品种
- 射击精灵，如苹果和蜂巢，可以把它们从树上打下来
- 拾取精灵的动画
- 增加VELOREN_ASSETS_OVERRIDE变量，用于指定文件夹以部分覆盖资产。
- 崇拜者突袭者
- 绽放滑块

### Changed

- Entity-entity pushback is no longer applied in forced movement states like rolling and leaping.
- Updated audio library (rodio 0.13 -> 0.14).
- Improve entity-terrain physics performance by reducing the number of voxel lookups.
- Clay Golem uses shockwave only after specific fraction of health and other difficulty adjustments.
- Made strafing slightly slower
- Food now has limited regeneration strength but longer duration.
- Harvester boss now has new abilities and AI
- Death particles and SFX
- Default keybindings were made more consistent
- Adjusted Yeti difficulty
- Now most of the food gives Saturation in the process of eating
- Mushroom Curry gives long-lasting Regeneration buff
- Trades now consider if items can stack in full inventories.
- The types of animals that can be tamed as pets are now limited to certain species, pending further balancing of pets
- Made server-cli admin add/remove command use positional arguments again
- Usage of "stamina" replaced with "energy"
- Glider dimensions now depend on character height
- Glider dimensions somewhat increased overall
- Dungeon difficulty level starts at 1 instead of 0
- The radius of the safe zone around the starting town has been doubled
- NPCs can sometimes drop no loot at all
- 实体-实体回推不再适用于滚动和跳跃等强制运动状态。
- 更新了音频库（rodio 0.13 -> 0.14）。
- 通过减少体素查找的次数来提高实体-地形物理学性能。
- 粘土巨魔只在特定的健康分数和其他难度调整后使用冲击波。
- 使得扫射的速度稍慢
- 食物现在具有有限的再生强度，但持续时间更长。
- 收割机老板现在有了新的能力和AI
- 死亡粒子和SFX
- 默认键位变得更加一致
- 调整了Yeti的难度
- 现在大多数食物在食用过程中会产生饱和度。
- 蘑菇咖哩提供了持久的再生BUFF
- 交易现在考虑到物品是否可以在完整的库存中堆积。
- 可以作为宠物驯服的动物类型现在仅限于某些物种，等待宠物的进一步平衡
- 使得server-cli管理员添加/删除命令再次使用位置参数。
- 用 "能量 "取代了 "体力 "的使用。
- 滑翔机的尺寸现在取决于角色的高度
- 滑翔机的尺寸在一定程度上增加了
- 地牢的难度等级从1开始，而不是0。
- 起始城镇周围安全区的半径增加了一倍。
- NPC有时可能根本不掉落战利品

### Removed

- Enemies no longer spawn in dungeon boss room
- Melee critical hit no longer applies after reduction by armour
- Enemies no more spawn in dungeon boss room
- Melee critical hit no more applies after reduction by armour
- Removed Healing Sceptre as a starting weapon as it is considered an advanced weapon
- The ability to pickup sprites through walls
- 敌人不再在地牢Boss房间中产生
- 近战关键一击不再适用于护甲减弱后的情况
- 敌人不再出现在地牢老板的房间里了
- 近战致命一击在被护甲削弱后不再适用
- 删除了治疗权杖作为初始武器，因为它被认为是一种高级武器。
- 通过墙壁拾取精灵的能力

### Fixed

- Crafting Stations aren't exploadable anymore
- Cases where no audio output could be produced before.
- Significantly improved the performance of playing sound effects
- Dismantle and Material crafting tabs don't have duplicated recipes
- Campfires now despawn when underwater
- Players no longer spawn underground if their waypoint is underground
- Map will now zoom around the cursor's position and drag correctly
- No more jittering while running down slopes with the glider out
- Axe normal attack rewards energy without skill points
- Gliders no longer suffer from unreasonable amounts of induced drag
- Camera is now clipping a lot less
- 工艺站不再是可扩展的了
- 以前不能产生音频输出的情况。
- 大幅提高了播放音效的性能
- 拆解和材料制作标签没有重复的配方了
- 篝火在水下时现在不再生了
- 如果玩家的航点在地下，他们不再在地下生成。
- 地图现在可以围绕光标的位置放大并正确拖动
- 在滑翔机伸出的情况下，在斜坡上奔跑时不会再出现抖动。
- 斧头的普通攻击会奖励能量而不需要技能点
- 滑翔机不再受到不合理的诱导阻力的影响
- 摄像机现在减少了剪切现象

## [0.10.0] - 2021-06-12

### Added

- New Skills for Climbing: Climbing Speed and Climbing Cost
- Pickaxes (can be used to collect gems and mine weak rock)
- You can now jump out of rolls for a slight jump boost
- Dungeons now have multiple kinds of stairs.
- Trades now display item prices in tooltips.
- Admin designated build areas
- Indicator text to collectable terrain sprites
- You can now autorequest exact change by ctrl-clicking in a trade, and can quick-add individual items with shift-click.
- Buy and sell prices in tooltips when trading with a merchant now have colors.
- Attacks now emit sound effects from the target on hit.
- Crafting menu tabs
- Auto camera setting, making the game easier to play with one hand
- Topographic map option
- Search bars for social and crafting window
- RTsim travellers now follow paths between towns
- "Poise" renamed to "Stun resilience"
- Stun resilience stat display
- Villagers and guards now spawn with potions, and know how to use them.
- Combat music in dungeons when within range of enemies.
- New Command: "kit", place a set of items into your inventory
- Added --sql-log-mode profile/trace parameter to veloren-server-cli
- Added /disconnect_all_players admin command
- Added disconnectall CLI command
- One handed weapons can now be used and found in the world
- Players can now opt-in to server-authoritiative physics in gameplay settings.
- Added `/server_physics` admin command.
- Sort inventory button
- Option to change the master volume when window is unfocused
- Crafting stations in towns
- Option to change the master volume when window is unfocused
- Entities now have mass
- Entities now have density
- Buoyancy is calculated from the difference in density between an entity and surrounding fluid
- Drag is now calculated based on physical properties
- Terrain chunks are now deflate-compressed when sent over the network.
- Missing translations can be displayed in English.
- New large birds npcs
- Day period dependant wildlife spawns
- You can now block and parry with melee weapons
- Lift is now calculated for gliders based on dimensions (currently same for all)
- Specific music tracks can now play exclusively in towns.
- Custom map markers can be placed now
- Fundamentals/prototype for wiring system
- Mountain peak and lake markers on the map
- There's now a checkbox in the graphics tab to opt-in to receiving lossily-compressed terrain colors.
- /buff command which allows you to cast a buff on player
- Warn the user with an animated red text in the second phase of a trade in which a party is offering nothing.
- /skill_preset command which allows you to apply skill presets
- Added timed bans and ban history.
- Added non-admin moderators with limit privileges and updated the security model to reflect this.
- Added a minimap mode that visualizes terrain within a chunk.
- Chat tabs
- NPC's now hear certain sounds
- Renamed Animal Trainers to Beastmasters and gave them their own set of armor to wear
- ChargedRanged attacks (such as some bow attacks) use an FOV zoom effect to indicate charge.
- Add chest to each dungeon with unique loot
- Added a new option in the graphics menu to enable GPU timing (not always supported). The timing values can be viewed in the HUD debug info (F3) and will be saved as chrome trace files in the working directory when taking a screenshot.
- Added new Present Mode option in the graphics menu. Selecting Fifo (i.e. vsync) or Mailbox can be used to eliminate screen tearing.
- Quality color indicators next to recipe names in crafting menu
- New cave visuals: Ridges, pits, new sprites, colors
- Veins in caves to dig through to uncover ore
- Armor material system with 6 armor sets each in hide, mail and cloth categories
- New armor stats including max energy, energy reward, critical hit damage
- Meat drops from animals
- New ores, plants and hides to be looted from the world and processed into craft ingredients
- Added more crafting stations, loom, spinning wheel, tanning rack, forge
- 攀登的新技能。攀登速度和攀登成本
- 镐头（可用于收集宝石和开采脆弱的岩石）
- 现在你可以从卷轴中跳出，获得轻微的跳跃提升
- 地下城现在有多种楼梯。
- 交易现在在工具提示中显示物品价格。
- 管理员指定的建造区域
- 可收集的地形精灵的指示文字
- 你现在可以通过在交易中点击ctrl来自动要求精确的变化，并可以通过点击shift来快速添加单个物品。
- 与商人交易时工具提示中的买卖价格现在有了颜色。
- 攻击现在可以发出目标被击中时的声音效果。
- 手工制作的菜单标签
- 自动相机设置，使游戏更容易用单手操作
- 地形图选项
- 社会和工艺窗口的搜索栏
- RTsim旅行者现在可以沿着城镇之间的路径行走了。
- "Poise "更名为 "Stun resilience"。
- 眩晕恢复力状态显示
- 村民和守卫现在都带着药水出生，并知道如何使用它们。
- 在地牢中进入敌人范围时的战斗音乐。
- 新指令。"套装"，将一组物品放入你的库存中
- 为 veloren-server-cli 添加了 --sql-log-mode profile/trace 参数
- 添加了 /disconnect_all_players 管理命令
- 增加了disconnectall CLI命令
- 现在可以在世界范围内使用和找到单手武器了
- 玩家现在可以在游戏设置中选择加入服务器授权的物理学。
- 添加了 `/server_physics` 管理命令。
- 库存排序按钮
- 当窗口不聚焦时，可以选择改变主音量
- 城镇中的工艺站
- 当窗口不聚焦时，可以选择改变主音量。
- 实体现在有了质量
- 实体现在有了密度
- 浮力是根据实体和周围液体的密度差计算的
- 阻力现在是根据物理属性计算的
- 地形块在通过网络发送时现在被压缩了。
- 缺少的翻译可以用英文显示。
- 新的大型鸟类npcs
- 取决于日间的野生动物产卵
- 你现在可以用近战武器进行格挡和招架了
- 现在根据尺寸计算滑翔机的升力（目前对所有滑翔机都一样）
- 特定的音乐曲目现在可以在城镇中专门播放。
- 现在可以放置自定义地图标记了
- 线路系统的基本原理/原型
- 地图上的山峰和湖泊标记
- 在图形标签中现在有一个复选框，可以选择接收有损压缩的地形颜色。
- /buff命令，允许你对玩家施放一个buff
- 在交易的第二阶段，如果一方没有提供任何东西，就会用红色的动画文字警告用户。
- /skill_preset命令，允许你应用技能预设。
- 增加了定时禁言和禁言历史。
- 增加了具有限制权限的非管理员版主，并更新了安全模式以反映这一点。
- 增加了最小地图模式，使大块内的地形可视化。
- 聊天标签
- NPC现在能听到某些声音
- 将动物训练师重新命名为兽人，并赋予他们自己的一套盔甲。
- 充电的远程攻击（如一些弓箭攻击）使用FOV缩放效果来表示充电。
- 在每个地牢中添加具有独特战利品的箱子
- 在图形菜单中增加了一个新的选项，以启用GPU计时（不总是支持）。计时值可以在HUD调试信息（F3）中查看，并且在截图时将作为chrome跟踪文件保存在工作目录中。
- 在图形菜单中增加了新的呈现模式选项。选择Fifo（即vsync）或Mailbox可以用来消除屏幕撕裂现象。
- 手工制作菜单中配方名称旁边的质量颜色指标
- 新的山洞视觉效果。山脊、坑洞、新的精灵、颜色
- 洞穴中的矿脉，可以通过挖掘来发现矿石
- 盔甲材料系统有6套盔甲，分别是皮甲、邮件和布甲。
- 新的盔甲统计，包括最大能量、能量奖励、关键打击伤害
- 从动物身上掉落的肉
- 新的矿石、植物和兽皮可以从世界上掠夺并加工成工艺原料
- 增加了更多的工艺站，织布机、纺车、制革架、锻造。

### Changed

- Admins can now grant normal players plots to place blocks within
- Diamonds are now much more than twice as expensive as twigs.
- Permission to build is no longer tied to being an admin
- Separated character randomization buttons into appearance and name.
- Reworked mindflayer to have unique attacks
- Glowing remains are now `Armor` instead of `Ingredients`.
- Generated a new world map
- Overhauled clouds for more verticality and performance
- New tooltip for items with stats comparison
- Improved bow feedback, added arrow particles
- Retiered most sceptres and staves
- Loot tables can now recursively reference loot tables
- "max_sfx_channels" default now set to 30
- Merchants now have stacks of stackable items instead of just one per slot
- Bag tooltips only show slots now
- Removed infinite armour values from most admin items
- Item tooltips during trades will now inform the user of what ctrl-click and shift-click do
- International keyboards can now display more key names on Linux and Windows instead of `Unknown`.
- There is now a brief period after a character leaves the world where they cannot rejoin until their data is saved
- Certain uses of client-authoritative physics now subject the player to server-authoritative physics.
- Dodge roll iframes and staff explosion are now unlocked by default, with points refunded for existing characters.
- Dash melee now stops after hitting something. Infinite dash also now replaced with dash through.
- Collisions, knockbacks, jumping and drag are now physical forces applied to the entity's body mass
- Turning rate has been made more consistent across angles
- Gravity has been lowered so that physics can work more reasonably
- Jump has been decreased in height but extended in length as a result of the new gravity
- Fall damage has been adjusted with the new gravity in mind
- Projectiles now generally have a different arc because they no longer have their own gravity modifier
- Increased agent system target search efficiency speeding up the server
- Added more parallelization to terrain serialization and removed extra cloning speeding up the server
- Energy now recharges while gliding
- Debug Kit is split to "admin_cosmetics" and "debug"
- Potion Kit is renamed to "consumables" and gives potions and mushroom curry
- Cultist Kit gives cape, rings and necklace in addition to armour and weapons.
- Reworked minotaur to have unique attacks.
- Wiring is now turing complete
- Better active/inactive master sound slider logic
- Cultist Husk no longer drops weapons and armor
- Animal Trainers now spawn in tier-5 dungeon and not in tier-3
- Reworked clay golem to have unique attacks.
- Merchants now use `/tell` instead of `/say` to communicate prices
- Entities catch on fire if they stand too close to campfires
- Water extinguishes entities on fire
- Item pickups are shown in separate window and inventory-full shows above item
- Reworked bow
- 管理员现在可以授予普通玩家地块来放置区块。
- 钻石现在比树枝贵了两倍多。
- 建筑许可不再与管理员身份挂钩。
- 将角色随机化按钮分为外观和名字。
- 重新设计了心灵杀手，使其具有独特的攻击力。
- 发光的残骸现在是 "盔甲 "而不是 "成分"。
- 产生了一个新的世界地图
- 为了提高垂直度和性能，对云层进行了大修。
- 为物品提供了新的工具提示，并进行了统计比较。
- 改进了弓的反馈，增加了箭的粒子
- 重置了大部分权杖和法杖
- 战利品表现在可以递归地引用战利品表
- max_sfx_channels "默认值现在设置为30
- 商人现在有了可堆叠的物品，而不是每个插槽只有一个。
- 袋子的工具提示现在只显示插槽
- 删除了大多数管理物品的无限护甲值
- 交易过程中的物品提示现在会告知用户点击ctrl和shift的作用。
- 国际键盘现在可以在Linux和Windows上显示更多的键名而不是 "未知"。
- 角色离开世界后，现在有一个短暂的时期，在他们的数据被保存之前，他们不能重新加入。
- 某些客户端授权物理学的使用现在使玩家受到服务器授权物理学的影响。
- 躲避卷轴iframe和法杖爆炸现在被默认解锁，现有角色的点数会被退还。
- 冲刺近战现在在击中东西后停止。无限冲刺现在也改为冲刺通过。
- 碰撞、击退、跳跃和拖拽现在都是应用于实体身体质量的物理力。
- 转弯率在不同角度下更加一致。
- 重力被降低了，以便物理学能够更合理地工作。
- 由于新的重力作用，跳跃的高度减少了，但长度延长了
- 在考虑到新重力的情况下，对坠落伤害进行了调整。
- 投射物现在一般有不同的弧度，因为它们不再有自己的重力修改器。
- 提高了代理系统的目标搜索效率，加快了服务器的速度
- 为地形序列化增加了更多的并行化，并删除了额外的克隆，加快了服务器的速度。
- 能量现在可以在滑翔时充能
- 调试工具包被拆分为 "admin_cosmetics "和 "debug"。
- 药水套装更名为 "消耗品"，提供药水和蘑菇咖喱。
- 崇拜者套装除了提供盔甲和武器外，还提供披风、戒指和项链。
- 重新制作了牛头人，使其拥有独特的攻击。
- 接线现在已经完成。
- 更好的主动/非主动主音滑块逻辑。
- 崇拜者之壳不再掉落武器和盔甲
- 动物训练师现在在5级地牢中生成，而不是在3级地牢中生成。
- 改造后的粘土巨魔有了独特的攻击。
- 商人现在使用 "tell "而不是 "say "来交流价格。
- 实体如果离篝火太近就会着火
- 水可以扑灭着火的实体
- 拾取的物品会显示在单独的窗口中，库存已满会显示上面的物品
- 重新制作了弓箭
- Switched to the `wgpu` graphics library giving us support for vulkan, dx12, metal, and dx11 (support for opengl is lost for the moment). This improves the graphics performance for many users.
- Reworked sprite rendering to vastly reduce the CPU work. Large sprite view distances are now much more performant.
- Optimized rendering of quads (most of the graphics in the game) using an index buffer, decreasing the number of vertices that need to be processed by 33%.
- Moved the rest of screenshot work into the background. Screenshoting no longer induces large pauses.
- Reworked tidal warrior to have unique attacks
- Reworked yeti to have unique attacks
- Widened recipe name list in crafting menu
- Reworked animal loot tables
- NPC hitboxes better fit their model.
- 切换到`wgpu`图形库，使我们支持vulkan、dx12、metal和dx11（暂时失去对opengl的支持）。这提高了许多用户的图形性能。
- 重做了精灵渲染，大大减少了CPU的工作。大的精灵视图距离现在更有性能。
- 使用索引缓冲器优化了四边形（游戏中的大部分图形）的渲染，将需要处理的顶点数量减少了33%。
- 将其余的截图工作移至后台。屏幕截图不再会引起大的停顿。
- 重做了潮汐战士，使其具有独特的攻击力。
- 重新设计了雪人，使其具有独特的攻击方式
- 拓宽了手工制作菜单中的配方名称列表
- 重新制作了动物战利品表
- NPC命中率更符合其模型。

### Removed

- Removed command: "debug", use "/kit debug" instead
- Gravity component has been removed
- In-air movement has been removed
- Energy cost of deploying the glider has been removed
- Removed steel and cultist loot tables
- 删除了命令。"debug"，使用"/kit debug "代替。
- 删除了重力组件
- 删除了空中运动
- 删除了部署滑翔机的能量成本
- 删除了钢铁和邪教的战利品表

### Fixed

- Server kicks old client when a user is trying to log in again (often the case when a user's original connection gets dropped)
- Added a raycast check to beams to prevent their effect applying through walls
- Flying agents raycast more angles to check for obstacles.
- Mouse Cursor now locks to the center of the screen when menu is not open
- Social window no longer moves when group is open
- Combat rating no longer takes buffs into account
- Minimap icons are now displayed in both map modes
- Server now denies any running trades when a user exits to the character selection screen.
- Sfx volume changes now also change the ambient sounds volume
- Staff fire shockwave ability no longer has an unlimited vertical range
- Skillbar buttons correctly account for skill points when checking if player has enough stamina for the ability.
- Burning Debuff icon is now displayed correctly.
- Villagers in safezones no longer spam messages upon seeing an enemy
- Wolf AI will no longer circle into walls and will instead use the power of raycasts to stop early
- Squirrels are no longer immune to arrows at some angles.
- /spawn command's auto-complete now works for species names
- Mindflayer AI now correctly summons husks at certain HP thresholds.
- Far away NPCs respond to being damaged by a projectile
- Fixed terrain clipping with glider
- Fixed an issue where prices weren't properly making their way from econsim to the actual trade values.
- Fixed entities with voxel colliders being off by one physics tick for collision.
- Airships no longer oscillate dramatically into the sky due to mistaking velocity for acceleration.
- The login and character selection screens no longer cause high GPU usage when the framerate limit is set to Unlimited.
- Deadwood will now attack targets who are at different elevations than itself.
- 当用户试图再次登录时，服务器会踢掉旧的客户端（当用户的原始连接被丢弃时经常出现这种情况）
- 为光束添加了射线传输检查，以防止其效果穿过墙壁。
- 飞行代理的光线照射有更多角度，以检查是否有障碍物。
- 当菜单未打开时，鼠标光标现在锁定在屏幕中央
- 群组打开时，社交窗口不再移动
- 战斗等级不再考虑BUFF了
- 最小地图图标现在可以在两种地图模式下显示了
- 当用户退出到角色选择界面时，服务器现在会拒绝任何正在进行的交易。
- 音量变化现在也会改变环境声音的音量
- 工作人员的火力冲击波能力不再具有无限的垂直范围
- 当检查玩家是否有足够的体力使用该能力时，技能栏按钮正确地考虑了技能点。
- 燃烧的Debuff图标现在可以正确显示。
- 安全区的村民在看到敌人时不再乱发信息。
- 狼的AI不再会绕到墙上，而是会使用射线的力量来提前停止。
- 松鼠不再对某些角度的箭矢有免疫力。
- /spawn命令的自动完成功能现在可以适用于物种名称。
- 弑心者AI现在可以正确地在特定的HP阈值下召唤出沙丘。
- 远处的NPC对被投射物伤害的反应。
- 修正了滑翔机的地形剪切问题
- 修正了一个问题，即价格不能正确地从econsim到实际的贸易价值。
- 修正了具有体素碰撞器的实体在碰撞时偏离一个物理刻度。
- 飞艇不再因为错把速度当做加速度而向天空急剧摆动。
- 当帧数限制设置为无限时，登录和角色选择屏幕不再导致GPU的高使用率。
- 枯木现在会攻击与自己处于不同海拔高度的目标。

## [0.9.0] - 2021-03-20

### Added

- Plugin can now retrieve data from ECS
- Added chat commands for inviting, kicking, leaving, and promoting in groups
- Aura system
- Campfire resting heal
- Initial support for game plugins, both server-side and client-side
- Reflective LoD water
- Map indicators for group members
- Hot-reloading for i18n, sounds, loot lotteries, and more
- Initial support for alternate style keyboards
- Flying birds travel the world
- Plugin system now based on Wasmer 1.0.0
- Added 4x Bag loadout slots, used for upgrading inventory space
- Added an additional Ring loadout slot
- The inventory can now be expanded to fill the whole window
- Added /dropall admin command (drops all inventory items on the ground)
- Skill trees
- Lactose tolerant golems
- 6 different gems. (Topaz, Amethyst, Sapphire, Emerald, Ruby and Diamond)
- Poise system
- Snow particles
- Basic NPC interaction
- Lights in dungeons
- Trading system (bound to the `R` key by default, currently only works with players)
- Support for dual wielding (not accessible as animations still needed)
- Support for modular weapons.
- Saturation buff (healing from food) now queues
- Coral reefs, kelp forests, and seagrass
- Talk animation
- New bosses in 5 lower dungeons
- New enemies in 5 lower dungeons
- Added on join event in plugins
- Item stacking and splitting
- Procedural trees (currently only oaks and pines are procedural)
- Cliffs on steep slopes
- Giant tree sites
- Reset button for graphics settings
- Gave weapons critical strike {chance, multiplier} stats
- A system to add glow and reflection effects to figures (i.e: characters, armour, weapons, etc.)
- Merchants will trade wares with players
- Airships that can be mounted and flown, and also walked on (`/airship` admin command)
- RtSim airships that fly between towns.
- 插件现在可以从ECS检索数据了
- 增加了群组中邀请、踢球、离开和推广的聊天命令
- 光环系统
- 篝火休息疗养
- 对游戏插件的初步支持，包括服务器端和客户端的插件
- 反射的LoD水
- 小组成员的地图指示器
- i18n、声音、战利品抽奖等的热重载
- 初步支持另一种风格的键盘
- 飞翔的鸟儿游遍世界
- 插件系统现在基于Wasmer 1.0.0
- 增加了4个袋子装载槽，用于升级库存空间
- 增加了一个额外的戒指装载槽
- 库存现在可以扩大到填满整个窗口
- 增加了/dropall管理命令（将所有库存物品丢在地上）。
- 技能树
- 容纳乳糖的戈尔人
- 6种不同的宝石。(黄宝石、紫水晶、蓝宝石、绿宝石、红宝石和钻石）。
- 姿态系统
- 冰雪微粒
- 基本的NPC互动
- 地牢中的灯光
- 交易系统（默认与`R`键绑定，目前只与玩家一起使用）
- 支持双持武器（由于仍然需要动画，所以无法进入）。
- 支持模块化武器。
- 饱和度buff（来自食物的治疗）现在可以排队了
- 珊瑚礁、海带林和海草
- 谈话动画
- 5个低级地牢中的新老板
- 5个低级地牢中的新敌人
- 在插件中增加了加入事件
- 物品堆叠和分割
- 程序化的树木（目前只有橡树和松树是程序化的）。
- 陡峭山坡上的悬崖
- 巨大的树址
- 重置图形设置的按钮
- 赋予武器关键打击{机会，乘数}统计。
- 为人物（如：人物、盔甲、武器等）添加发光和反射效果的系统。
- 商人将与玩家交易商品
- 可以安装和飞行的飞艇，也可以在上面行走（`/airship`管理命令）。
- RtSim的飞艇可以在城镇之间飞行。

### Changed

- Doubled range of ScaleMode slider when set to Custom
- Glider can now be deployed mid-air at the cost of some stamina based on fall speed
- Translations are now folders with multiple files instead of a huge single file
- Default inventory slots reduced to 18 - existing characters given 3x 6-slot bags as compensation
- Protection rating was moved to the top left of the loadout view
- Changed camera smoothing to be off by default.
- Footstep SFX is now dependant on distance moved, not time since last play
- Adjusted most NPCs hitboxes to better fit their models.
- Changed crafting recipes involving shiny gems to use diamonds instead.
- Cave scatter now includes all 6 gems.
- Adjusted Stonework Defender loot table to remove mindflayer drops (bag, staff, glider).
- Made humanoid NPCs use gliders (if equipped) when falling
- Changed default controller key bindings
- Improved network efficiency by ≈ factor 10 by using tokio.
- Added item tooltips to trade window.
- "Quest" given to new players converted to being a short tutorial
- Items can be requested from the counterparty's inventory during trade.
- Savanna grasses restricted to savanna, cacti to desert.
- Fireworks recursively shoot more fireworks.
- Improved static light rendering and illumination
- Improved the tree spawning model to allow for overlapping forests
- Changed sunlight (and, in general, static light) propagation through blocks to allow for more material properties
- Overhauled the sceptre
- Make the /time command relative to the current day
- Spatial partitioning via a grid for entity versus entity collisions was added which can more than halve the total tick time at higher entity counts (> ~1000)
- Improved efficency of entity versus terrain collisions (they now take less than half the time)
- The loading screen will now display random animations
- 当设置为自定义时，ScaleMode滑块的范围增加了一倍。
- 滑翔机现在可以在半空中展开，但要根据下降速度消耗一些体力。
- 翻译现在是带有多个文件的文件夹，而不是一个巨大的单一文件。
- 默认库存槽减少到18个--现有角色获得了3个6槽包作为补偿。
- 保护等级被移到了装备视图的左上方
- 改变了摄像机的平滑度，默认为关闭。
- 脚步声SFX现在取决于移动的距离，而不是上次游戏后的时间。
- 调整了大多数NPC的命中率，以更好地适应他们的模型。
- 改变了涉及闪亮宝石的工艺配方，以使用钻石代替。
- 洞穴散落物现在包括所有6种宝石。
- 调整了石器防御者的战利品表，删除了弑心者的掉落物（袋子、法杖、滑翔机）。
- 使得人形NPC在坠落时使用滑翔机（如果有装备）。
- 改变了默认的控制器按键绑定方式
- 通过使用tokio提高了网络效率≈系数10。
- 在交易窗口增加了物品工具提示。
- 给新玩家的 "任务 "改成了简短的教程
- 在交易过程中可以从对方的库存中索取物品。
- 稀树草原的草只限于稀树草原，仙人掌只限于沙漠。
- 焰火可以递归地发射更多的焰火。
- 改进了静态光的渲染和照度
- 改进了树木产卵模型，允许重叠的森林。
- 改变了阳光（以及一般来说，静态光）通过块的传播，以允许更多的材料属性。
- 大修了权杖
- 使/时间命令与当前日期相对应
- 为实体与实体之间的碰撞增加了通过网格的空间划分，在较高的实体数量（> ~1000）下，可以将总的勾选时间减少一半以上。
- 提高了实体与地形碰撞的效率（它们现在只需要不到一半的时间）
- 加载屏幕现在将显示随机动画

### Removed

- SSAAx4 option
- The Stats button and associated screen were removed
- Levels
- Shiny Gems (replaced with diamonds)
- SSAAx4选项
- 删除了 "统计 "按钮和相关屏幕
- 级别
- 闪亮的宝石（用钻石代替）

### Fixed

- Fixed a bug that would cause a server crash when a player levelled up or fired
  a projectile in very specific circumstances
- Fixed a bug where buff/debuff UI elements would flicker when you had more than
  one of them active at the same time
- Made zooming work on wayland
- Fixed AI behavior so only humanoids will attempt to roll
- Fixed missing GameInputs (sneak, swimup, swimdown) in controller mapping
- Fixed missing controller actions (dance and crafting)
- Fixed a bug where the stairs to the boss floor in dungeons would sometimes not spawn
- Fixed waypoints being placed underwater
- Objects and golems are not affected by bleed debuff anymore
- Fixed RtSim entity memory loss
- Mandated that merchants not wander away during a trade
- Fixed the villager conception of evil by encouraging them to react violently to characters wearing cultist gear
- 修正了一个错误，在特定情况下，当玩家提升等级或发射弹丸时，会导致服务器崩溃。
  在非常特殊的情况下会导致服务器崩溃
- 修正了一个bug，当你有多个buff/debuff同时激活时，UI元素会闪动。
  的时候，BUFF/DEBUFF的UI元素会闪烁
- 使得缩放功能可以在路边使用
- 修正了人工智能的行为，使之只有人形生物才会尝试翻滚
- 修复了控制器映射中缺失的GameInputs（潜行、游上、游下）。
- 修正了缺失的控制器动作（舞蹈和手工制作）。
- 修正了地下城中通往老板层的楼梯有时无法生成的错误。
- 修正了航点被放置在水下的问题
- 物体和巨兽不再受流血减伤的影响
- 修复了RtSim实体记忆丢失的问题
- 规定商人在交易过程中不得随意走动
- 修正了村民对邪恶的概念，鼓励他们对穿着邪教装备的角色做出激烈的反应。

## [0.8.0] - 2020-11-28

### Added

- New level of detail feature, letting you see all the world's terrain at any view distance.
- Point and directional lights now cast realistic shadows, using shadow mapping.
- Added leaf and chimney particles
- Some more combat sound effects
- Beehives and bees
- Fireflies
- Fullscreen modes now show two options (exclusive and borderless)
- Added banlist and `/ban`, `/unban`, and `/kick` commands for admins
- A new dungeon boss (venture there and discover it yourself)
- Adaptive stride setup for more dynamic run behavior
- Theropod body
- Several new animals
- Item quality indicators
- Added a jump/burst attack for the bow to the skillbar
- Gave the axe a third attack
- A new secondary charged melee attack for the hammer
- Added Dutch translations
- Buff system
- Sneaking lets you be closer to enemies without being detected
- Flight
- Roll dodges melee attacks, and reduces the height of your hitbox
- Persistent waypoints (start from the last camp fire you visited)
- NPCs use all three weapon skills in combat
- Speed stat to weapons which affects weapon attack speed
- Saving of the last selected character in the character selection screen
- Autoselecting the newly created character
- Deselecting when the selected character is deleted
- Upscaling support
- Added "Persist Combo from Combo Melee State" when rolling mid-combo
- You can no longer spam hammer and bow special when stamina is 0
- Biome and site specific music system
- Ambient SFX emitted from terrain blocks
- Campfire SFX
- Wind SFX system
- Added Norwegian language
- Roll can now interrupt attacks
- Birch forests
- Willow forests
- More significant temperature variation across the world
- Initial implementation of real-time world simulation
- Travellers that explore the world
- HDR rendering
- Map site icons
- Map panning
- Innumerable minor improvements to world generation
- Variable dungeon difficulty
- Aurora Borealis (localised entirely within the kitchen)
- Block-based voxel lighting
- Animals now have customized attacks and AI
- 新的详细程度功能，让你在任何视距都能看到世界上所有的地形。
- 使用阴影贴图，点状和方向性灯光现在可以投下真实的阴影。
- 增加了树叶和烟囱粒子
- 增加了一些战斗声音效果
- 蜂巢和蜜蜂
- 萤火虫
- 全屏模式现在显示两个选项（独占和无边界）。
- 为管理员增加了禁止名单和"/禁止"、"/取消 "和"/踢 "命令
- 一个新的地牢老板（在那里冒险，自己发现）。
- 自适应步幅设置，使运行行为更有活力
- 翼手类动物的身体
- 几个新的动物
- 物品质量指示器
- 在技能栏中增加了弓箭的跳跃/爆裂攻击
- 给予斧头第三次攻击
- 为锤子增加了新的二次带电近战攻击
- 增加了荷兰语的翻译
- 增强系统
- 潜行让你更接近敌人而不被发现
- 飞行
- 滚动可以躲避近战攻击，并降低你的攻击框的高度
- 持久航点（从你最后访问的营火开始）。
- NPC在战斗中使用所有三种武器技能
- 对武器进行速度统计，影响武器攻击速度
- 在角色选择屏幕中保存最后选择的角色
- 自动选择新创建的角色
- 当所选角色被删除时取消选择
- 支持缩放比例
- 增加了在连击中滚动时的 "从连击近战状态持续连击"。
- 当体力为0时，你不能再使用锤子和弓的特殊技能了
- 生物群落和场地特定的音乐系统
- 从地形块发出的环境SFX
- 篝火SFX
- 风声SFX系统
- 增加了挪威语
- 滚动现在可以打断攻击了
- 桦树林
- 柳树森林
- 整个世界的温度变化更加明显
- 初步实现了实时世界模拟
- 探索世界的旅行者
- HDR渲染
- 地图站点图标
- 地图平移
- 对世界的生成进行了无数次的小改进
- 可变的地牢难度
- 北极光（完全在厨房内局部）。
- 基于块的体素照明
- 动物现在有自定义的攻击和AI

### Changed

- The world map has been refactored to support arbitrary sizes and compute horizon maps.
- Veloren's lighting has been completely overhauled.
- The graphics options were made much more flexible and configurable.
- Many shader optimizations.
- Voxel model creation was switched to use greedy meshing, improving performance.
- Animation and terrain math were switched to use SIMD where possible, improving performance.
- The way we cache glyphs was refactored, fixed, and optimized.
- Colors for models and figures were adjusted to account for the saturation hack.
- Overhauled world colours
- Improved projectile physics
- Improved overhead aiming
- Improved first person aiming
- Figure meshing no longer blocks the main thread.
- Overhauled persistence layer including no longer storing serialized JSON items in the database
- Overhauled representation of blocks to permit fluid and sprite coexistence
- Overhauled sword
- Reworked healing sceptre
- Split out the sections of the server settings that can be edited and saved by the server.
- Revamped structure of where settings, logs, and game saves are stored so that almost everything is in one place.
- Moved hammer leap attack to skillbar
- Reworked fire staff
- Overhauled cloud shaders to add mist, light attenuation, an approximation of rayleigh scattering, etc.
- Allowed collecting nearby blocks without aiming at them
- Made voxygen wait until singleplayer server is initialized before attempting to connect, removing the chance for it to give up on connecting if the server takes a while to start
- Log where userdata folder is located
- Switched to a Whittaker map for better tree spawning patterns
- Switched to procedural snow cover on trees
- Significantly improved terrain generation performance
- Significantly stabilized the game clock, to produce more "constant" TPS
- Transitioned main menu and character selection screen to a using iced for the ui (fixes paste keybinding on macos, removes password field limits, adds tabbing between input fields in the main menu, adds language selection in the main menu)
- Made settings less likely to reset when the format changes
- Adjusted some keybindings
- Consumables can now trigger multiple effects and buffs
- Overhauled overworld spawns depending on chunk attributes
- Improved cloud and water shader quality
- 世界地图已经被重构，以支持任意尺寸和计算地平线地图。
- 维洛伦的照明已经被彻底改造。
- 图形选项变得更加灵活和可配置。
- 许多着色器得到了优化。
- 沃克斯尔模型的创建被转换为使用贪婪网格划分，从而提高了性能。
- 动画和地形数学被转换为尽可能使用SIMD，以提高性能。
- 我们对字形的缓存方式进行了重构、修复和优化。
- 调整了模型和人物的颜色，以考虑到饱和度的问题。
- 大修了世界的颜色
- 改进了投射物物理学
- 改进了高空瞄准
- 改进了第一人称瞄准
- 人物网格划分不再阻塞主线程。
- 大修了持久层，包括不再在数据库中存储序列化的JSON项目。
- 大修了块的表示方法，允许流体和精灵共存。
- 对剑进行了大修
- 重新设计了治疗权杖
- 分离了可由服务器编辑和保存的服务器设置部分。
- 修改了设置、日志和游戏保存的结构，以便几乎所有东西都在一个地方。
- 将锤子跳跃攻击移至技能栏
- 重新设计了火杖
- 大修了云层着色器，增加了雾气、光线衰减、雷利散射的近似值，等等。
- 允许收集附近的方块而无需瞄准它们
- 让voxygen在尝试连接前等待单人服务器初始化，消除了在服务器启动时放弃连接的可能性。
- 记录用户数据文件夹的位置
- 切换到Whittaker地图，以获得更好的树木产卵模式。
- 改用程序化的树木雪盖
- 大幅提高了地形生成的性能
- 大大稳定了游戏时钟，以产生更 "稳定 "的TPS。
- 将主菜单和角色选择屏幕过渡到使用冰的界面（修复了Macos上的粘贴键绑定，删除了密码字段限制，在主菜单的输入字段之间添加了标签，在主菜单中添加了语言选择）。
- 当格式改变时，设置不太可能被重置
- 调整了一些键位的绑定
- 消耗品现在可以触发多种效果和BUFF
- 根据大块的属性，重新修改了世界上的产卵。
- 改进了云和水的着色器质量

### Removed

- MSAA has been removed due to incompatibility with greedy meshing.
- Removed a saturation hack that led to colors being improperly displayed.
- 由于与贪婪网格划分不兼容，已经删除了MSAA。
- 删除了一个导致颜色显示不当的饱和度黑客。

### Fixed

- Fixed a bug where leaving the Settings menu by pressing "N" in single player kept the game paused.
- Fixed a bug where the closest item would be picked up instead of a selected item.
- Fixed a bug where camera zoom in and zoom out distance didn't match.
- Fixed a bug where a nearby item would also be collected when collecting collectible blocks
- Fixed a bug where firing fast projectile at a downwards angle caused them to veer off at a higher angle
- Fixed a bug where ui scale in the login menu was not updated when changed in-game
- Fixed a bug which caused campfires and other stuff to duplicate
- Significantly improved water movement AI to stop entities getting stuck
- Prevented entities, sprites and particles being lit when not visible to the sun
- 更正了一个错误，在单人游戏中，按 "N "离开设置菜单时，游戏会一直暂停。
- 更正了一个错误，即最近的物品会被拾起，而不是被选中的物品。
- 更正了一个相机放大和缩小距离不一致的错误。
- 修正了一个错误，即在收集可收集块时，附近的物品也会被收集。
- 修正了一个错误，即以向下的角度发射快速弹丸会导致它们以更高的角度偏离。
- 修正了一个错误，在游戏中改变登录菜单中的UI比例时，不会更新。
- 修正了一个导致篝火和其他东西重复的错误。
- 大幅改进了水的运动AI，以阻止实体被卡住。
- 防止实体、精灵和粒子在不被太阳看到时被点亮

## [0.7.0] - 2020-08-15

### Added

- Display item name over loot/dropped items
- Added Lottery system for loot
- Added context-sensitive crosshair
- Announce alias changes to all clients
- Dance animation
- Speech bubbles appear when nearby players talk
- NPCs call for help when attacked
- Eyebrows and shapes can now be selected
- Character name and level information to chat, social tab and `/players` command
- Added inventory, armour and weapon saving
- Show where screenshots are saved in the chat
- Added basic auto walk
- Added weapon/attack sound effects
- M2 attack for bow
- Hotbar persistence
- Alpha version Disclaimer
- Server whitelist
- Optional server-side maximum view distance
- MOTD on login
- Added group chat `/join_group` `/group`
- Added faction chat `/join_faction` `/faction`
- Added regional, local, and global chat (`/region`, `/say`, and `/world`, respectively)
- Added command shortcuts for each of the above chat modes (`/g`, `/f`, `/r`, `/s`, and `/w`, respectively and `/t` for `/tell`)
- Ability to wield 2 × 1h weapons and shields (Note: 1h weapons & shields are not currently avaliable, see [!1095](https://gitlab.com/veloren/veloren/-/merge_requests/1095) for more info)
- Zoomable Map
- M2 attack for hammer
- Spawnable training dummies
- New quadruped_low body for reptile-likes
- Added new animals
- Better pathfinding
- Bombs
- Training dummy items
- Added spin attack for axe
- Creature specific stats
- Minimap compass
- Initial crafting system implementation
- Protection stat to armor that reduces incoming damage
- Loading-Screen tips
- Feeding animation for some animals
- Power stat to weapons which affects weapon damage
- Add detection of entities under the cursor
- Functional group-system with exp-sharing and disabled damage to group members
- Some Campfire, fireball & bomb; particle, light & sound effects.
- Added firework recipe
- Added setting to change resolution
- Rare (unfinished) castles
- Caves with monsters and treasure
- Furniture and decals in towns
- 在战利品/掉落物品上显示物品名称
- 增加了战利品的抽奖系统
- 增加了上下文敏感的十字准线
- 向所有客户宣布别名的变化
- 舞蹈动画
- 当附近的玩家说话时，会出现语音气泡
- NPC在受到攻击时呼救
- 现在可以选择眉毛和形状
- 聊天、社交标签和`/玩家`命令中的角色名称和等级信息
- 增加了库存、盔甲和武器的保存
- 在聊天中显示屏幕截图的保存位置
- 增加了基本的自动行走
- 增加了武器/攻击的声音效果
- 弓箭的M2攻击
- 热键持久性
- 阿尔法版免责声明
- 服务器白名单
- 可选的服务器端最大视距
- 登录时的MOTD
- 增加了群组聊天`/join_group `/group`。
- 增加了派别聊天 `/join_faction` `/faction`。
- 增加了区域、本地和全球聊天（分别为`/region`、`/say`和`/world`）。
- 增加了上述每种聊天模式的命令快捷键（分别为`/g`, `/f`, `/r`, `/s`, 和`/w`, 以及`/t`用于`/tell`）。
- 能够挥舞2×1小时的武器和盾牌（注意：1小时的武器和盾牌目前还不能使用，更多信息见[!1095](https://gitlab.com/veloren/veloren/-/merge_requests/1095)
- 可放大的地图
- 锤子的M2攻击
- 可再生的训练假人
- 为喜欢爬行动物的人增加了新的四足动物_低体。
- 增加了新的动物
- 更好的寻路功能
- 炸弹
- 训练用假人物品
- 增加了斧头的旋转攻击
- 生物的具体统计信息
- 小地图指南针
- 初始制作系统的实施
- 盔甲的保护状态，减少传来的伤害
- 加载屏幕提示
- 某些动物的喂食动画
- 影响武器伤害的武器动力状态
- 增加对光标下实体的检测
- 功能性的团体系统，可以分享经验，并对团体成员造成无效伤害
- 一些篝火、火球和炸弹；粒子、光和声音效果。
- 增加了烟花配方
- 增加了改变分辨率的设置
- 罕见的（未完成的）城堡
- 有怪物和宝藏的洞穴
- 城镇中的家具和贴纸

### Changed

- Improved camera aiming
- Made civsim, sites, etc. deterministic from the same seed
- Improved animations by adding orientation variation
- new tail bone for quad_small body
- slim the game size through lossless asset optimization
- Lanterns now stop glowing if you throw a lit one out of your inventory
- Fixed a crash caused by certain audio devices on OSX
- Bow animations now show held arrows
- Fixed a bug where walk/run sfx played while a character rolled/dodged
- Energy regen resets on last ability use instead of on wield
- Fixed unable to use ability; Secondary and ability3 (fire rod) will now automatically wield
- Gliding is now a toggle that can be triggered from the ground
- Replaced `log` with `tracing` in all crates
- Switch to a new network backend that will allow several improvements in the future
- Connection screen fails after 4 minutes if it can't connect to the server instead of 80 minutes
- Rebuilt quadruped_medium/quadruped_small animation and assets
- Disabled destruction of most blocks by explosions
- Disable damage to pets
- Made pets healable
- Rebalanced fire staff
- Animals are more effective in combat
- Pathfinding is much smoother and pets are cleverer
- Animals run/turn at different speeds
- Updated windowing library (winit 0.19 -> 0.22)
- Bow M2 is now a charged attack that scales the longer it's held
- Fixed window resizing on Mac OS X.
- Dehardcoded many item variants
- Tooltips avoid the mouse better and disappear when hovered
- Improved social window functions and visuals
- Changed agent behaviour to allow fleeing
- Waypoints now spawn on dungeon staircases
- 改进相机瞄准
- 使公民模拟、场地等从同一种子中确定。
- 通过增加方向的变化改进了动画效果
- 为四体的小型机体增加了新的尾部骨骼
- 通过无损资产优化，缩小了游戏尺寸
- 如果你把点燃的灯笼从库存中扔出去，它现在就不会再发光了
- 修正了OSX上某些音频设备引起的崩溃。
- 弓箭的动画现在可以显示持有的箭
- 修正了一个错误，即当角色翻滚/躲避时，会播放行走/奔跑音效。
- 能量再生在最后一次使用能力时重置，而不是在挥动时重置
- 修正了无法使用能力的问题；次级能力和能力3（火棒）现在会自动挥舞。
- 滑翔现在是一个可以从地面上触发的开关。
- 在所有箱子中用 "追踪 "取代了 "记录"。
- 切换到新的网络后台，这将允许在未来进行一些改进
- 如果不能连接到服务器，连接屏幕会在4分钟后失效，而不是80分钟。
- 重建了四足动物_中/四足动物_小的动画和资产
- 禁用了爆炸对大多数区块的破坏
- 禁用对宠物的伤害
- 使得宠物可以得到治疗
- 重新平衡了火杖
- 动物在战斗中更加有效
- 寻路更顺畅了，宠物更聪明了
- 动物以不同的速度运行/转弯
- 更新了窗口库（winit 0.19 -> 0.22）。
- 弓箭M2现在是一种带电攻击，持有时间越长，攻击力越大
- 修正了Mac OS X上的窗口大小调整问题。
- 删除了许多物品变体的编码
- 工具提示更好地避开了鼠标，悬停时消失了
- 改进了社交窗口的功能和视觉效果
- 改变了代理人的行为，允许其逃离
- 路径点现在在地牢的楼梯上产生。

### Removed

- Wield requirement to swap loadout; fixes issue with unable swap loadout outside of combat
- Disclaimer wall of text on first startup
- 交换装备的挥舞要求；修复了在战斗之外无法交换装备的问题
- 首次启动时的免责声明墙

## [0.6.0] - 2020-05-16

### Added

- Added music system
- Added zoomable and rotatable minimap
- Added rotating orientation marker to main-map
- Added daily Mac builds
- Allow spawning individual pet species, not just generic body kinds
- Configurable fonts
- Configurable keybindings from the Controls menu
- Translation status tracking
- Added gamma setting
- Added new orc hairstyles
- Added SFX for wielding/unwielding weapons
- Fixed NPCs attacking the player forever after killing them
- Added SFX for collecting, dropping and using inventory items
- New attack animation
- Weapon control system
- Game pauses when in single player and pause menu
- Added authentication system (to play on the official server register on <https://account.veloren.net>)
- Added gamepad/controller support
- Added player feedback when attempting to pickup an item with a full inventory
- Added free look
- Added Italian translation
- Added Portuguese translation
- Added Turkish translation
- Added Traditional Chinese translation
- Complete rewrite of the combat system into a state machine
- Abilities like Dash and Triplestrike
- Armor can now be equipped as items
- Fireball explosions
- Inventory supports stacking
- Many new armors and weapons to find in chests
- Fleshed out "attack" animation into alpha, beta and spin type attacks
- Fleshed out range attack into charging and shooting animations for staff/bow
- Customized attack animation for hammers and axes
- Added German translation
- Added a silhouette for players when they are occluded
- Added transparency to the player when zooming in
- Made armor and hotbar slots actually function
- Added dragging and right-click to use functionality to inventory, armor & hotbar slots
- Added capes, lanterns, tabards, rings, helmets & necklaces as equippable armor
- 6 new music tracks
- Added basic world and civilization simulation
- Added overhauled towns
- Added fields, crops and scarecrows
- Added paths
- Added bridges
- Added procedural house generation
- Added lampposts
- Added NPCs that spawn in towns
- Added simple dungeons
- Added sub-voxel noise effect
- Added waypoints next to dungeons
- Made players spawn in towns
- Added non-uniform block heights
- Added `/sudo` command
- Added a Level of Detail (LoD) system for terrain sprites and entities
- Added owl, hyena, parrot, cockatrice, red dragon NPCs
- Added dungeon entrances
- Villagers tools and clothing
- Cultists clothing
- You can start the game by pressing "enter" from the character selection menu
- Added server-side character saving
- Player now starts with a lantern. Equipping/unequipping a lantern has the same effect as the `/lantern` command
- Added tab completion in chat for player names and chat commands
- Added server persistence for character stats
- Added a popup when setting your character's waypoint
- Added dungeon arenas
- Added dungeon bosses and rare boss loot
- Added 2 sets of armour. One Steel and one Leather.
- 增加了音乐系统
- 增加了可缩放和可旋转的小地图
- 在主地图上添加了旋转方向的标记
- 添加了每日Mac构建
- 允许生成单个宠物物种，而不仅仅是通用的身体种类
- 可配置的字体
- 从控制菜单中可配置的键盘绑定
- 翻译状态跟踪
- 增加了伽玛设置
- 增加了新的兽人发型
- 增加了挥舞/解除武器的SFX音效
- 修正了NPC在杀死后永远攻击玩家的问题
- 增加了收集、掉落和使用库存物品的音效
- 新的攻击动画
- 武器控制系统
- 在单人游戏和暂停菜单中，游戏会暂停
- 增加了认证系统（要在官方服务器上玩，请注册<https://account.veloren.net>）。
- 增加了游戏手柄/控制器支持
- 增加了试图在满载的情况下拾取物品时的玩家反馈
- 增加了自由外观
- 添加了意大利语翻译
- 添加了葡萄牙语翻译
- 添加了土耳其语翻译
- 添加了繁体中文翻译
- 将战斗系统完全重写为一个状态机
- 冲刺和三连击等能力
- 盔甲现在可以作为物品装备
- 火球爆炸
- 库存支持叠加
- 许多新的盔甲和武器可以在箱子里找到。
- 将 "攻击 "动画充实到阿尔法、贝塔和旋转类型的攻击中。
- 丰富了法杖/弓箭的范围攻击，变成了充电和射击动画
- 定制了锤子和斧子的攻击动画
- 增加了德语翻译
- 增加了玩家被遮挡时的剪影
- 增加了玩家放大时的透明度
- 增加了盔甲和热键槽的实际功能
- 为库存、盔甲和热键槽添加了拖动和右键使用的功能
- 增加了披风、灯笼、帐幕、戒指、头盔和项链作为可装备的盔甲
- 6首新的音乐曲目
- 增加了基本的世界和文明模拟
- 增加了大修后的城镇
- 添加了田地、农作物和稻草人
- 增加了道路
- 添加了桥梁
- 添加了程序化的房屋生成
- 添加了灯柱
- 添加了在城镇中产生的NPC
- 添加了简单的地牢
- 添加了子体素噪音效果
- 在地牢旁边添加了航点
- 使得玩家在城镇中产卵
- 添加了非统一的块状高度
- 增加了"/sudo "命令
- 为地形精灵和实体添加了细节等级（LoD）系统
- 添加了猫头鹰、土狼、鹦鹉、鸡冠花、红龙等NPC
- 增加了地牢的入口
- 村民的工具和服装
- 崇拜者的服装
- 可以在角色选择菜单中按 "回车 "开始游戏
- 增加了服务器端的角色保存
- 玩家现在开始有一个灯笼。装备/取消装备灯笼的效果与`/灯笼`命令相同。
- 在聊天中为玩家名字和聊天命令增加了标签完成功能
- 增加了人物统计的服务器持久性
- 增加了设置角色航点时的弹出窗口
- 增加了地牢竞技场
- 增加了地牢老板和稀有老板的战利品
- 增加了2套盔甲。一套钢制，一套皮革。

### Changed

- The /give_item command can now specify the amount of items. Syntax is now `/give_item <name> [num]`
- Brighter / higher contrast main-map
- Removed highlighting of non-collectible sprites
- Fixed /give_exp ignoring player argument
- Extend run sfx to small animals to prevent sneak attacks by geese.
- Decreased clientside latency of ServerEvent mediated effects (e.g. projectiles, inventory operations, etc)
- Started changing the visual theme of the UI
- Merge of the Bag and Character-Screen
- Merge of the Map and Questlog
- Overhauled icon art
- Asset cleanup to lower client-size
- Rewrote the humanoid skeleton to be more ideal for attack animations
- Arrows can no longer hurt their owners
- Increased overall character scale
- `/sudo player /tp` is short for `/sudo player /tp me`
- The `/object` command can create any object in comp::object::Body
- The `/help` command takes an optional argument. `/help /sudo` will show you information about only the sudo command.
- /give_item命令现在可以指定物品的数量。语法现在是`/give_item <name> [num]`。
- 更加明亮/高对比度的主地图
- 删除了对不可收集的精灵的高亮显示
- 修正了 /give_exp 忽略玩家参数的问题
- 将运行音效扩展到小动物，以防止鹅的偷袭。
- 减少了由ServerEvent介导的效果（如射弹、库存操作等）的客户端延迟。
- 开始改变用户界面的视觉主题
- 合并袋子和角色屏幕
- 合并地图和任务日志
- 大修了图标艺术
- 清理资产以降低客户端尺寸
- 重写人形骨架，使其更适合于攻击动画的制作
- 箭头不再能伤害其主人
- 增加了角色的整体比例
- `/sudo player /tp`是`/sudo player /tp me`的缩写。
- `/object`命令可以创建comp::object::Body中的任何对象。
- `/help`命令需要一个可选的参数。`/help /sudo`将只显示关于sudo命令的信息。

## [0.5.0] - 2020-01-31

### Added

- Added new debug item
- Bows give experience by projectiles having an owner
- Allow cancelling chunk generation
- Include licence in assets
- Added dropping items
- Added initial region system implementation
- Added /giveitem command
- Strip Linux executables
- Added moon
- Added clouds
- Added tarpaulin coverage
- Added ability to jump while underwater
- Added proper SFX system
- Added changelog
- Added animated Map and Minimap position indicator
- Added visuals to indicate strength compared to the player
- Added Scrolling Combat Text (SCT) & Settings for it
- Added a Death Screen and Hurt Screen
- Added randomly selected Loading Screen background images
- Added options to disable clouds and to use cheaper water rendering
- Added client-side character saving
- Added a localization system to provide multi-language support
  to voxygen
- Added French language for Voxygen
- Added rivers and lakes which follow realistic physical paths.
- Added a sophisticated erosion system for world generation which
  dramatically changes the world layout.
- Added tracking of sediment vs. bedrock, which is visually reflected in the
  world.
- Added map saving and loading for altitude and bedrock, with built in
  versioning for forwards compatibility.
- Added a default map, which is used to speed up starting single player.
- Added a 3D renderered map, which is also used by the server to send the map
  to the client.
- Added fullscreen and window size to settings so that they can be persisted
- Added coverage based scaling for pixel art
- 28 new mobs
- Added waypoints
- Added pathfinding to NPCs
- Overhauled NPC AI
- Pets now attack enemies and defend their owners
- Added collars to tame wild animals
- 增加了新的调试项目
- 弓箭通过有主人的弹丸提供经验
- 允许取消块的生成
- 在资产中包括许可证
- 增加掉落物品
- 增加了初始区域系统的实现
- 增加了/giveitem命令
- 剥离Linux可执行文件
- 添加了月亮
- 添加了云层
- 添加了油布覆盖
- 增加了在水下跳跃的能力
- 添加了适当的SFX系统
- 添加了更新日志
- 添加了动画地图和迷你地图位置指示器
- 增加了显示与玩家相比实力的视觉效果
- 添加了滚动战斗文本（SCT）及其设置
- 增加了死亡画面和受伤画面
- 增加了随机选择的加载屏幕背景图片
- 增加了禁用云层和使用更便宜的水渲染的选项
- 增加了客户端的角色保存功能
- 添加了一个本地化系统，以提供多语言支持
  到沃克西根
- 为Voxygen添加了法语语言
- 添加了河流和湖泊，它们遵循真实的物理路径。
- 增加了一个复杂的世界生成的侵蚀系统，可以大大改变世界的布局。
  显著地改变了世界的布局。
- 增加了对沉积物与基岩的跟踪，这在世界中得到了直观的反映。
  世界。
- 增加了对海拔和基岩的地图保存和加载，并有内置的
  版本的兼容性。
- 增加了一个默认地图，用来加快单人游戏的启动速度。
- 增加了一个3D渲染的地图，服务器也用它来发送地图
  到客户端。
- 在设置中增加了全屏和窗口尺寸，以便它们可以被持久化。
- 为像素艺术添加了基于覆盖率的缩放功能
- 增加了28个新的暴徒
- 添加了航点
- 为NPC添加了寻路功能
- 大修了NPC的AI
- 宠物现在可以攻击敌人并保护它们的主人
- 增加了驯服野生动物的项圈

### Changed

- Controls pane in settings window now shows actual configured keys
- Fixed scroll wheel and roll keys on OS X
- Fixed near and far view planes
- Improvements to armor names
- Animation fixes to line up with true positions
- Proper message for command permission check failure
- Improved meshing
- Improved dusk
- Improved movement and climbing
- Improved water rendering and chunk render order
- Moved computations to terrain fragment shaders
- Fixed title music
- Made rolling less violent when changing directions
- Fixed single player crash
- Improved error information in client and server
- Store items as RON files
- Updated download info in readme
- Fixed cloud performance
- Fixed region display name
- Fixed the bow fire rate
- Healthbars now flash on critical health
- Fixed ghosts when going back to character screen
- Fixed not being able to unmount
- Fixed non-humanoids being able to climb and glide
- Made shadows and lights use interpolated positions
- Changed "Create Character" button position
- Made clouds bigger, more performant and prettier
- Terrain meshing optimized further
- Tree leaves no longer color blended
- Actual character stats displayed in character window
- Made significant changes to the noise functions used for world generation.
- Improved colors during world generation.
- Significantly reduced the use of warp during world generation.
- Parallelized and otherwise sped up significant parts of world generation.
- Various performance improvements to world generation.
- Nametags now a fixed size and shown in a limited range
- Non-humanoid skeletons now utilize configs for hotloading, and skeletal attributes.
- Names of NPCs spawned in the wild now include their species.
- 设置窗口中的控制窗格现在显示实际配置的按键
- 修正了OS X上的滚轮和滚动键
- 修复了近景和远景平面
- 改进了盔甲名称
- 修复了动画，使之与真实位置一致
- 命令权限检查失败时的正确信息
- 改进了网格划分
- 改进了黄昏
- 改进了移动和爬升
- 改进了水的渲染和分块渲染顺序
- 将计算移到地形碎片着色器上
- 修正了标题音乐
- 改变方向时使滚动不那么剧烈
- 修正了单人游戏的崩溃
- 改进了客户端和服务器中的错误信息
- 将物品存储为RON文件
- 更新了自述中的下载信息
- 修正了云计算性能
- 修正了区域显示名称
- 修正了弓箭的发射率
- 健康条现在会在临界健康时闪烁
- 修正了回到角色界面时的幽灵问题
- 修正了无法卸载的问题
- 修正了非人类能够攀爬和滑翔的问题
- 使阴影和灯光使用内插位置
- 改变了 "创建角色 "按钮的位置
- 使得云层更大、性能更强、更漂亮
- 进一步优化了地形网格化
- 树叶不再混合颜色
- 在角色窗口中显示了实际的角色统计信息
- 对用于生成世界的噪声函数进行了重大修改。
- 改进了世界生成过程中的颜色。
- 大大减少了世界生成过程中对翘曲的使用。
- 平行化并以其他方式加快了世界生成的重要部分。
- 对世界生成的各种性能进行了改进。
- 名片现在有了固定的尺寸，并在一个有限的范围内显示。
- 非人形骨架现在可以利用配置进行热加载，以及骨架属性。
- 在野外产生的NPC的名字现在包括他们的物种。

### Removed

- Remove heaptrack as it is now deprecated
- 移除heaptrack，因为它现在已被废弃。

## [0.4.0] - 2019-10-10

### Added

- Added adjustable FOV slider
- Added /explosion command
- Added first person switch
- Added single player server settings
- Added admin check for commands
- Started asset reloading system
- Added SRGB conversion in shaders
- Added adminify to give temp admin privilages
- - 增加了可调的FOV滑块
- 增加了/爆炸命令
- 增加了第一人称切换
- 添加了单人服务器设置
- 增加了管理员对命令的检查
- 开始了资产重载系统
- 在着色器中添加了SRGB转换
- 增加了管理权限，给临时的管理权限。

### Changed

- Collision and fall damage fixes
- Switched to eventbus system
- Improved seed generation, diffusion function
- Switch to hashbrown in server/client
- Improved colors and lighting
- Replaced view distance culling with frustum culling
- 碰撞和坠落损伤的修复
- 切换到事件总线系统
- 改进种子生成、扩散功能
- 在服务器/客户端中切换到哈希布朗系统
- 改进了颜色和照明
- 用地壳剔除取代了视距剔除

## [0.3.0] - 2019-08-04

### Added

- Added enemies
- Added player info to debug window
- Added server info
- Game settings persist after closing
- Added caves
- Added random NPC names
- Added tree roots, houses, basic lights
- Added XP and leveling
- Added build mode
- Character customization, multiple races
- Inventories (WIP)
- Day/night, better shaders, voxel shadows
- 添加了敌人
- 在调试窗口添加了玩家信息
- 添加了服务器信息
- 游戏设置在关闭后仍然有效
- 增加了山洞
- 增加了随机NPC名称
- 增加了树根、房屋、基本灯光
- 增加了XP和等级提升
- 增加了建造模式
- 角色定制，多种族
- 库存(WIP)
- 日/夜，更好的着色器，体素阴影

### Changed

- Fixed attack delay
- Fixed disclaimer to show only once
- Only send physics updates for entities within view distance
- Fix for headphones and invalid device parameters
- Fixed asset names for consistancy
- Fixes animals jumping after their target no matter how far
- Improved SFX in caves
- Better combat, movement, and animations
- Many performance optimizations
- Better world generation, more biomes
- 修正了攻击延迟
- 修正免责声明，只显示一次
- 只对视距内的实体发送物理学更新
- 修复了耳机和无效的设备参数
- 修正了资产名称的一致性
- 修复了动物无论多远都会在目标后跳跃的问题
- 改进了山洞中的SFX
- 更好的战斗、运动和动画
- 许多性能优化
- 更好的世界生成，更多的生物群落

## [0.2.0] - 2019-05-28

### Added

- Hang Gliding 悬挂式滑翔机
- Pets: Pig and Wolf. They can be spawned with /pig and /wolf commands.
- 宠物。猪和狼。它们可以通过/pig和/wolf命令生成。
- Name tags: You can finally know who is this guy with the blue shirt!
- 姓名标签。你终于可以知道这个穿蓝衬衫的家伙是谁了
- single player: No need to start a server just to play alone
- 单人游戏。不需要为了单人游戏而启动一个服务器
- Character customization: It isn't fully complete but still allows you to look different than others
- 角色定制。它并不完全完整，但仍然可以让你看起来与其他人不同
- Music!
- Major performance improvements related to the fact that we rewrote the entire game
- 与我们重写了整个游戏有关的主要性能改进
- 0% chance to get a deadlock
- 0%的机会出现死锁
- Animations: You finally can move your limbs!
- 动画。你终于可以移动你的四肢了
- Combat: You can finally swing your sword that has been on your back. Enemies are coming soon, but you can always fight with other players
- 战斗：你终于可以挥舞一直在你背上的剑了。敌人即将到来，但你可以随时与其他玩家战斗。
- When a server dies the game no longer crashes - you will be just kicked to the main menu
- 当服务器死亡时，游戏不再崩溃--你将被踢到主菜单。

## [0.1.0] - 2018-XX-XX

_0.1.0 was part of the legacy engine_

[unreleased]: https://gitlab.com/veloren/veloren/compare?from=v0.13.0&to=master
[0.13.0]: https://gitlab.com/veloren/veloren/compare?from=v0.12.0&to=v0.13.0
[0.12.0]: https://gitlab.com/veloren/veloren/compare?from=v0.11.0&to=v0.12.0
[0.11.0]: https://gitlab.com/veloren/veloren/compare?from=v0.10.0&to=v0.11.0
[0.10.0]: https://gitlab.com/veloren/veloren/compare?from=v0.9.0&to=v0.10.0
[0.9.0]: https://gitlab.com/veloren/veloren/compare?from=v0.8.0&to=v0.9.0
[0.8.0]: https://gitlab.com/veloren/veloren/compare?from=v0.7.0&to=v0.8.0
[0.7.0]: https://gitlab.com/veloren/veloren/compare?from=v0.6.0&to=v0.7.0
[0.6.0]: https://gitlab.com/veloren/veloren/compare?from=v0.5.0&to=v0.6.0
[0.5.0]: https://gitlab.com/veloren/veloren/compare?from=v0.4.0&to=v0.5.0
[0.4.0]: https://gitlab.com/veloren/veloren/compare?from=v0.3.0&to=v0.4.0
[0.3.0]: https://gitlab.com/veloren/veloren/compare?from=v0.2.0&to=v0.3.0
[0.2.0]: https://gitlab.com/veloren/veloren/compare?from=7d17f8b67a2a6d5aa00730f028cedc430fd5075a&to=v0.2.0
[0.1.0]: https://gitlab.com/veloren/game
