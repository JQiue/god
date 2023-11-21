use std::{env::temp_dir, fs::File, io::Write};
use storage::{get_keyboard, get_mouse};
use webbrowser::open;

pub fn preview() {
  let keyboard_data = get_keyboard();
  let mouse_data = get_mouse();
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
  <div id="mouse" style="height: 100%;"></div>

  <script>
    const keyboardDom = document.getElementById('keyboard');
    const mouseDom = document.getElementById('mouse');
    const keyboardChart = echarts.init(keyboardDom);
    const mouseChart = echarts.init(mouseDom);
    const keyboardChartOption = {{
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
    const mouseChartOption = {{
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
    keyboardChart.setOption(keyboardChartOption);
    mouseChart.setOption(mouseChartOption);
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
    mouse_data.iter().map(|(a, _b)| a).collect::<Vec<&String>>(),
    mouse_data.iter().map(|(_a, b)| b).collect::<Vec<&i32>>(),
  );

  let mut file = File::create(&html_path).unwrap();
  file
    .write_all(content.as_bytes())
    .expect("Failed to write to file");
  drop(file);

  open(html_path.to_str().expect("error")).expect("error")
}
