pub mod activity {
  use std::{env::temp_dir, fs::File, io::Write};
  use webbrowser::open;

  use crate::storage::storage::{get_keyboard, get_keyboard1, get_mouse, get_mouse1};

  pub fn preview() {
    let keyboard_data = get_keyboard();
    let keyboard1_data = get_keyboard1();
    let mouse_data = get_mouse();
    let mouse1_data = get_mouse1();
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
  <div id="mouse" style="height: 100%;"></div>
  <div id="mouse1" style="height: 100%;"></div>

  <script>
    const keyboardDom = document.getElementById('keyboard');
    const keyboard1Dom = document.getElementById('keyboard1');
    const mouseDom = document.getElementById('mouse');
    const mouse1Dom = document.getElementById('mouse1');

    const keyboardChart = echarts.init(keyboardDom);
    const keyboard1Chart = echarts.init(keyboard1Dom);
    const mouseChart = echarts.init(mouseDom);
    const mouse1Chart = echarts.init(mouse1Dom);

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
        text: "本周每天按键次数",
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
        text: "本周每天点击次数",
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

    keyboardChart.setOption(keyboardChartOption);
    keyboard1Chart.setOption(keyboard1ChartOption);
    mouseChart.setOption(mouseChartOption);
    mouse1Chart.setOption(mouse1ChartOption);

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
      mouse_data.iter().map(|(a, _b)| a).collect::<Vec<&String>>(),
      mouse_data.iter().map(|(_a, b)| b).collect::<Vec<&i32>>(),
      mouse1_data
    );

    let mut file = File::create(&html_path).unwrap();
    file
      .write_all(content.as_bytes())
      .expect("Failed to write to file");
    drop(file);

    open(html_path.to_str().expect("error")).expect("error")
  }
}
