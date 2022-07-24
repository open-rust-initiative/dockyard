export const generat_options=(names,data)=>{
   return {
        title: {
            text: 'Nightingale Chart',
            subtext: 'on this current page',
                left: 'left'
        },
        tooltip: {
            trigger: 'item',
                formatter: '{b} : {c} MiB ({d}%)'
        },
        legend: {
            left: 'center',
                top: 'bottom',
                data: names
        },
        series: [
            {
                name: 'Storage space used',
                type: 'pie',
                radius: [40, 140],
                center: ['50%', '60%'],
                roseType: 'radius',
                itemStyle: {
                    borderRadius: 3
                },
                label: {
                    show: true
                },
                emphasis: {
                    label: {
                        show: true
                    }
                },
                data: data
            },
        ]
    }
}