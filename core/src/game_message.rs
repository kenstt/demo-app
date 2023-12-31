use rand::Rng;

pub fn message_factory() -> String {
    let messages = vec![
        "這關真的難啊！",
        "我得好好策略一下。",
        "等等，我差點就贏了！",
        "看來我需要改變策略。",
        "我要試試看這個技能。",
        "哎呀，差一點點就贏了。",
        "這遊戲的設計真是精妙。",
        "我覺得我找到了一個漏洞。",
        "這關的音樂太讚了！",
        "我得找找任務日誌，看看下一步該怎麼辦。",
        "不好，我的生命值快沒了！",
        "等待時間好長啊，我可以打個小遊戲嗎？",
        "終於找到出口了！",
        "我要嘗試一下這個新裝備。",
        "這場比賽真刺激！",
        "我得等一下，我的技能冷卻中。",
        "這遊戲的圖像效果真是令人驚艷。",
        "我忘記了我放在哪兒的地圖。",
        "下一關應該會更難吧。",
        "這個任務有點繁複。",
        "我要加入一個隊伍，一起合作。",
        "看來我需要更多的經驗才能解鎖這個區域。",
        "這遊戲的故事情節真是扣人心弦。",
        "我得回城去裝備更好的道具。",
        "這個遊戲的地圖真大啊！",
        "我差一點就達成成就了。",
        "等待時間好無聊，有什麼新消息嗎？",
        "我想挑戰一下其他玩家。",
        "這個遊戲的遊戲性真是太棒了。",
        "我們需要更好的協調，才能贏得這場比賽。",
        "這個遊戲的藝術風格真是獨特。",
        "我要試試看這個技巧是否適用。",
        "我需要一些額外的資源來升級裝備。",
        "有人知道這個關卡的暗道在哪嗎？",
        "這個 BOSS 真是難纏啊！",
        "我忘了怎麼使用這個技能了。",
        "看來我得去賺些金幣。",
        "這個遊戲的地圖設計真是令人驚嘆。",
        "有沒有人願意幫我完成這個團隊任務？",
        "我需要更多的經驗才能解鎖新的技能。",
        "我要嘗試一下不同的角色建構。",
        "這個遊戲的音效效果太逼真了！",
        "我得找到一個安全的地方回復生命值。",
        "等待時間真是折磨人啊。",
        "這個遊戲的 PVP 真是激烈。",
        "我差一點就通過這個關卡了。",
        "有人知道這個怪物的攻擊模式嗎？",
        "我得收集更多的材料來打造新裝備。",
        "這個遊戲的社群真是友好。",
        "我需要提升我的技能才能應對更高難度的挑戰。",
    ];
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..messages.len());
    messages[index].to_string()
}