pub mod activity {
  use std::{env::temp_dir, fs::File, io::Write, time::Instant};
  use webbrowser::open;

  use crate::storage::storage::{
    get_keyboard, get_keyboard1, get_keyboard2, get_mouse, get_mouse1, get_mouse2,
  };

  pub fn preview() {
    let mut now = Instant::now();
    let keyboard_data = get_keyboard();
    println!("keyboard_data: {:?}", now.elapsed());
    now = Instant::now();
    let keyboard1_data = get_keyboard1();
    println!("keyboard1_data: {:?}", now.elapsed());
    now = Instant::now();
    let keyboard2_data = get_keyboard2();
    println!("keyboard2_data: {:?}", now.elapsed());
    now = Instant::now();
    let mouse_data = get_mouse();
    println!("mouse_data: {:?}", now.elapsed());
    now = Instant::now();
    let mouse1_data = get_mouse1();
    println!("mouse1_data: {:?}", now.elapsed());
    now = Instant::now();
    let mouse2_data = get_mouse2();
    println!("mouse2_data: {:?}", now.elapsed());

    now = Instant::now();
    let temp_dir = temp_dir();
    // 输出 HTML 路径
    let out_html_path = temp_dir.join("index.html");
    let out_js_path = temp_dir.join("index.js");
    let js_content = include_str!("./asserts/index.js");
    let content = format!(
      r#"
      <!DOCTYPE html>
      <html lang="en" style="height: 100%;">
      
      <head>
      <title>God</title>
      <script src="https://cdn.bootcdn.net/ajax/libs/echarts/5.4.3/echarts.js"></script>
      <script src="./index.js"></script>
      </head>
      
      <body style="height: 100%;">
      <div id="keyboard" style="height: 50vh;"></div>
      <div style="display: flex;">
      <div id="keyboard1" style="display: inline-block; width: 50%;height: 50vh;"></div>
      <div id="keyboard2" style="display: inline-block; width: 50%;height: 50vh;"></div>
      </div>
      <div style="display: flex;">
      <div id="mouse" style="display: inline-block; width: 50%;height: 50vh;"></div>
      <div id="mouse1" style="display: inline-block; width: 50%;height: 50vh;"></div>
      <div id="mouse2" style="display: inline-block; width: 50%;height: 50vh;"></div>
      </div>
      
      <script>
      const keyboardChart = echarts.init(document.getElementById('keyboard'), 'wonderland');
      const keyboard1Chart = echarts.init(document.getElementById('keyboard1'), 'wonderland');
      const keyboard2Chart = echarts.init(document.getElementById('keyboard2'), 'wonderland');
      const mouseChart = echarts.init(document.getElementById('mouse'), 'wonderland');
      const mouse1Chart = echarts.init(document.getElementById('mouse1'), 'wonderland');
      const mouse2Chart = echarts.init(document.getElementById('mouse2'), 'wonderland');
      
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
          text: "本周按键次数",
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
          text: "本周点击次数",
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

    let mut html_file = File::create(&out_html_path).unwrap();
    html_file
      .write_all(content.as_bytes())
      .expect("Failed to write to file");
    let mut js_file = File::create(&out_js_path).unwrap();
    js_file
      .write_all(js_content.as_bytes())
      .expect("Failed to write to file");

    drop(html_file);
    drop(js_file);

    open(out_html_path.to_str().expect("error")).expect("error");
    println!("out_html: {:?}", now.elapsed());
  }
}
