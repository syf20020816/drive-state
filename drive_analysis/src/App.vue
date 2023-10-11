<template>
  <div class="app">
    <div ref="chart" id="chart" class="chart"></div>
    <div class="btn_wrap">

      <input type="text" :value="dir_path" style="width: 68%;">
      <button @click="get_dirs">加载数据</button>
      <button @click="drawChart">刷新</button>
      <button @click="next">下一个</button>
    </div>
  </div>
</template>



<script lang="ts" setup>
import { ref, reactive, onMounted } from 'vue'
import * as echarts from 'echarts'
import { invoke } from '@tauri-apps/api/tauri'
type EChartsOption = echarts.EChartsOption
let dir_path = ref('E:\\Rust\\try\\auto_drive_all\\datas\\reoverlays')
let pointer = ref(-1)
let dirs = reactive([])
let chart = ref(null)
let myChart: any
let chart_datas = reactive({
  node: [] as Array<any>,
  link: [] as Array<any>
})

const get_dirs = async () => {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  dirs = await invoke('get_dir', { dirPath: dir_path.value })
}
// get_dirs()

const next = () => {
  echarts.dispose(myChart)
  chart_datas.node.splice(0, chart_datas.node.length)
  chart_datas.link.splice(0, chart_datas.link.length)
  pointer.value++

  invoke('get_data', { path: dir_path.value + '\\' + dirs[pointer.value] }).then((res: any) => {
    let tmpData = JSON.parse(res)
    //上一个状态，用于比较
    let pre = { id: 0, status: 'Normal', action: null, time: 0, sign: null } as {
      id: number
      status: any
      action: any
      time: number
      sign: any
    }

    let counter = 0
    for (let i = 0; i < tmpData.length; i++) {
      for (let j = 0; j < tmpData[i].length; j++) {
        // 上一次和这一次没变->状态没变
        if (tmpData[i][j].action == pre.action && tmpData[i][j].status == pre.status) {
        } else {
          // 状态改变
          // 记录上一个状态和当前状态
          let y = 100
          if (pre.time != tmpData[i][j].time) {
            y = 500
          }
          //判断pre是否为object
          let preNodeName = pre.status
          if (typeof pre.status === 'object' && pre.status !== null) {
            preNodeName = pre.status.Other
          }
          let action_str = tmpData[i][j].action == null ? 'null' : tmpData[i][j].action
          if (typeof tmpData[i][j].action === 'object' && tmpData[i][j].action !== null) {
            action_str = tmpData[i][j].action.Other
          }
          let color = '#75db79'
          if (pre.status != 'Normal') {
            color = '#db7575'
          }

          let tmp = {
            node: {
              name: preNodeName + pre.time.toFixed(3) + '|' + chart_datas.node.length,
              x: 180 * counter,
              y: y,
              itemStyle: {
                color: color
              },
              label: {
                color: '#fff'
              }
            },
            link: {
              // source: JSON.stringify(pre.status) + pre.time.toFixed(3),
              // target: JSON.stringify(tmpData[i][j].status) + tmpData[i][j].time.toFixed(3),
              source: chart_datas.node.length,
              target: chart_datas.node.length + 1,
              label: {
                show: true,
                formatter: action_str
              },
              lineStyle: {
                curveness: 0.2
              }
            }
          }

          chart_datas.node.push(tmp.node)
          chart_datas.link.push(tmp.link)
          counter++
        }
        pre = tmpData[i][j]
      }
    }
    chart_datas.node.push({
      name: pre.status + 'END',
      x: (180 * counter) / 2,
      y: 800
    })
  })
}

const drawChart = () => {
  console.log(chart_datas.node)
  console.log(chart_datas.link)
  let chartDom = document.getElementById('chart')!
  myChart = echarts.init(chartDom)
  let option: EChartsOption

  option = {
    title: {
      text: '状态检测'
    },
    tooltip: {},
    width: '6000px',
    grid: {
      width: '6000px'
    },
    textStyle: {
      color: '#fff'
    },
    animationDurationUpdate: 1500,
    animationEasingUpdate: 'quinticInOut',
    series: [
      {
        type: 'graph',
        layout: 'none',
        symbolSize: 50,
        roam: true,
        label: {
          show: true
        },
        edgeSymbol: ['circle', 'arrow'],
        edgeSymbolSize: [4, 10],
        edgeLabel: {
          fontSize: 16
        },
        data: chart_datas.node,
        links: chart_datas.link,
        lineStyle: {
          opacity: 0.9,
          width: 2,
          curveness: 0
        }
      }
    ]
  }

  option && myChart.setOption(option)
}

onMounted(() => {})
</script>

<style  scoped>
.app {
  height: 100%;
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-wrap: wrap;
}
.chart {
  height: 80%;
  width: 100%;
  overflow-x: scroll;
}

canvas {
  width: 6000px;
}

.btn_wrap {
  min-height: 66px;
  height: 20%;
  width: 100%;
  display: flex;
  align-content: center;
  align-items: center;
  justify-content: center;
  flex-wrap: wrap;
}
button {
  background: #757ddb;
  margin: 4px 20px;
}
</style>