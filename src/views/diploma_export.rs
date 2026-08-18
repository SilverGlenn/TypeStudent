use std::fs;
use std::path::PathBuf;

pub fn export_diploma_html(student_name: &str, test_title: &str, net_wpm: f32, accuracy: f32, date: &str, duration: &str) -> Option<PathBuf> {
    let html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Certificate of Touch Typing Achievement - {student_name}</title>
    <style>
        @page {{ size: landscape; margin: 0; }}
        body {{
            font-family: 'Georgia', serif;
            background: #fdfbf7;
            display: flex;
            justify-content: center;
            align-items: center;
            min-height: 100vh;
            margin: 0;
            padding: 30px;
            box-sizing: border-box;
        }}
        .certificate {{
            width: 860px;
            padding: 40px;
            border: 10px solid #b45309;
            outline: 3px solid #f59e0b;
            outline-offset: -7px;
            background: #ffffff;
            text-align: center;
            box-shadow: 0 10px 30px rgba(0,0,0,0.1);
        }}
        .badge {{ font-size: 54px; margin-bottom: 8px; }}
        h1 {{
            color: #78350f;
            font-size: 34px;
            text-transform: uppercase;
            letter-spacing: 2px;
            margin: 10px 0;
        }}
        .subtitle {{
            color: #92400e;
            font-size: 15px;
            font-style: italic;
            margin-bottom: 20px;
        }}
        .recipient {{
            font-size: 36px;
            color: #0369a1;
            font-weight: bold;
            border-bottom: 2px solid #cbd5e1;
            display: inline-block;
            padding: 0 40px 8px 40px;
            margin: 10px 0 20px 0;
        }}
        .reason {{
            color: #475569;
            font-size: 16px;
            margin-bottom: 25px;
        }}
        .stats-box {{
            display: flex;
            justify-content: center;
            gap: 30px;
            margin: 20px 0;
        }}
        .stat {{
            background: #fef3c7;
            border: 1px solid #fde68a;
            padding: 12px 30px;
            border-radius: 8px;
        }}
        .stat-val {{
            font-size: 26px;
            font-weight: bold;
            color: #92400e;
        }}
        .stat-lbl {{
            font-size: 11px;
            color: #78350f;
            text-transform: uppercase;
            letter-spacing: 1px;
        }}
        .footer {{
            display: flex;
            justify-content: space-between;
            margin-top: 40px;
            padding: 0 30px;
            color: #64748b;
            font-size: 13px;
        }}
        .sign {{
            border-top: 1px solid #94a3b8;
            padding-top: 5px;
            font-weight: bold;
            color: #334155;
        }}
    </style>
</head>
<body>
    <div class="certificate">
        <div class="badge">🏆</div>
        <h1>Certificate of Touch Typing Achievement</h1>
        <div class="subtitle">TypeStudent Pro Official Examination</div>
        <p style="color: #64748b; font-size: 15px;">This certifies that</p>
        <div class="recipient">{student_name}</div>
        <div class="reason">has successfully passed the examination: <strong>{test_title}</strong> ({duration})</div>
        <div class="stats-box">
            <div class="stat"><div class="stat-val">{net_wpm:.1} WPM</div><div class="stat-lbl">Net Speed</div></div>
            <div class="stat"><div class="stat-val">{accuracy:.1}%</div><div class="stat-lbl">Accuracy</div></div>
        </div>
        <div class="footer">
            <div class="sign">Date: {date}</div>
            <div class="sign">TypeStudent Examination Board 📜</div>
        </div>
    </div>
</body>
</html>"#);

    let path = PathBuf::from("diploma_export.html");
    if fs::write(&path, html).is_ok() {
        Some(path)
    } else {
        None
    }
}
