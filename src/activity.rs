pub mod activity {
  use std::{env::temp_dir, fs::File, io::Write};
  use webbrowser::open;

  use crate::storage::storage::{
    get_keyboard, get_keyboard1, get_keyboard2, get_mouse, get_mouse1, get_mouse2,
  };

  pub fn preview() {
    let keyboard_data = get_keyboard();
    let keyboard1_data = get_keyboard1();
    let keyboard2_data = get_keyboard2();
    let mouse_data = get_mouse();
    let mouse1_data = get_mouse1();
    let mouse2_data = get_mouse2();
    let temp_dir = temp_dir();
    let html_path: std::path::PathBuf = temp_dir.join("index.html");
    let content = format!(
      r#"
  <!DOCTYPE html>
<html lang="en" style="height: 100%;">

<head>
  <title>God</title>
  <script src="https://cdn.bootcdn.net/ajax/libs/echarts/5.4.3/echarts.js"></script>
</head>

<body style="height: 100%;">
  <div id="keyboard" style="height: 100%;"></div>
  <div id="keyboard1" style="height: 100%;"></div>
  <div id="keyboard2" style="height: 100%;"></div>
  <div id="mouse" style="height: 100%;"></div>
  <div id="mouse1" style="height: 100%;"></div>
  <div id="mouse2" style="height: 100%;"></div>

  <script>
    const keyboardChart = echarts.init(document.getElementById('keyboard'));
    const keyboard1Chart = echarts.init(document.getElementById('keyboard1'));
    const keyboard2Chart = echarts.init(document.getElementById('keyboard2'));
    const mouseChart = echarts.init(document.getElementById('mouse'));
    const mouse1Chart = echarts.init(document.getElementById('mouse1'));
    const mouse2Chart = echarts.init(document.getElementById('mouse2'));

    const keyboardChartOption = {{
      title: {{
        text: "键码次数",
        left: "center"
      }},
      tooltip: {{
        trigger: 'axis',
        axisPointer: {{
          type: 'shadow'
        }}
      }},
      xAxis: {{
        type: 'category',
        data: {:?}
      }},
      yAxis: {{
        type: 'value'
      }},
      series: [
        {{
          data: {:?},
          type: 'bar'
        }}
      ]
    }};

    const keyboard1ChartOption = {{
      title: {{
        text: "本周每日按键次数",
        left: "center"
      }},
      tooltip: {{
        trigger: 'axis',
        axisPointer: {{
          type: 'shadow'
        }}
      }},
      xAxis: {{
        type: 'category',
        data: ["星期一", "星期二", "星期三", "星期四", "星期五", "星期六", "星期天"]
      }},
      yAxis: {{
        type: 'value'
      }},
      series: [
        {{
          data: {:?},
          type: 'bar'
        }}
      ]
    }};

    const keyboard2ChartOption = {{
      title: {{
        text: "每月按键次数",
        left: "center"
      }},
      tooltip: {{
        trigger: 'axis',
        axisPointer: {{
          type: 'shadow'
        }}
      }},
      xAxis: {{
        type: 'category',
        data: ["一月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "十二月"]
      }},
      yAxis: {{
        type: 'value'
      }},
      series: [
        {{
          data: {:?},
          type: 'bar'
        }}
      ]
    }};

    const mouseChartOption = {{
      title: {{
        text: "鼠标次数",
        left: "center"
      }},
      tooltip: {{
        trigger: 'axis',
        axisPointer: {{
          type: 'shadow'
        }}
      }},
      xAxis: {{
        type: 'category',
        data: {:?}
      }},
      yAxis: {{
        type: 'value'
      }},
      series: [
        {{
          data: {:?},
          type: 'bar'
        }}
      ]
    }};

    const mouse1ChartOption = {{
      title: {{
        text: "本周每日点击次数",
        left: "center"
      }},
      tooltip: {{
        trigger: 'axis',
        axisPointer: {{
          type: 'shadow'
        }}
      }},
      xAxis: {{
        type: 'category',
        data: ["星期一", "星期二", "星期三", "星期四", "星期五", "星期六", "星期天"]
      }},
      yAxis: {{
        type: 'value'
      }},
      series: [
        {{
          data: {:?},
          type: 'bar'
        }}
      ]
    }};

    const mouse2ChartOption = {{
      title: {{
        text: "每月点击次数",
        left: "center"
      }},
      tooltip: {{
        trigger: 'axis',
        axisPointer: {{
          type: 'shadow'
        }}
      }},
      xAxis: {{
        type: 'category',
        data: ["一月", "二月", "三月", "四月", "五月", "六月", "七月", "八月", "九月", "十月", "十一月", "十二月"]
      }},
      yAxis: {{
        type: 'value'
      }},
      series: [
        {{
          data: {:?},
          type: 'bar'
        }}
      ]
    }};

    keyboardChart.setOption(keyboardChartOption);
    keyboard1Chart.setOption(keyboard1ChartOption);
    keyboard2Chart.setOption(keyboard2ChartOption);
    mouseChart.setOption(mouseChartOption);
    mouse1Chart.setOption(mouse1ChartOption);
    mouse2Chart.setOption(mouse2ChartOption);

    window.addEventListener('resize', keyboardChart.resize);
  </script>
</body>

</html>
  "#,
      keyboard_data
        .iter()
        .map(|(a, _b)| a)
        .collect::<Vec<&String>>(),
      keyboard_data.iter().map(|(_a, b)| b).collect::<Vec<&i32>>(),
      keyboard1_data,
      keyboard2_data,
      mouse_data.iter().map(|(a, _b)| a).collect::<Vec<&String>>(),
      mouse_data.iter().map(|(_a, b)| b).collect::<Vec<&i32>>(),
      mouse1_data,
      mouse2_data
    );

    let mut file = File::create(&html_path).unwrap();
    file
      .write_all(content.as_bytes())
      .expect("Failed to write to file");
    drop(file);

    open(html_path.to_str().expect("error")).expect("error")
  }
}
