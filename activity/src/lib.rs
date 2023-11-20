use std::{env::temp_dir, fs::File, io::Write};
use storage::get_keyboard;
use webbrowser::open;

pub fn preview() {
  let data = get_keyboard();
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
  <div id="main" style="height: 100%;"></div>

  <script>
    const chartDom = document.getElementById('main');
    const myChart = echarts.init(chartDom);
    const option = {{
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
    }}
    myChart.setOption(option);
    window.addEventListener('resize', myChart.resize);
  </script>
</body>

</html>
  "#,
    data.iter().map(|(a, _b)| a).collect::<Vec<&String>>(),
    data.iter().map(|(_a, b)| b).collect::<Vec<&i32>>(),
  );

  let mut file = File::create(&html_path).unwrap();
  file
    .write_all(content.as_bytes())
    .expect("Failed to write to file");
  drop(file);

  open(html_path.to_str().expect("error")).expect("error")
}
