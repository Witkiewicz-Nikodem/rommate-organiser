const group_name = document.getElementById("group_name").innerHTML;
document.getElementById("group_name").remove();

async function load_charts(group_name){
    try{
        const response = await fetch(
            'https://rust-app-production.up.railway.app/expenses/group/summed/' + group_name,{
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                }
        });
        if(response.ok){
            const data = await response.json();
            const chart_labels = [];
            const chart_data = [];
            data.forEach(element => {
                chart_labels.push(element[0])
                chart_data.push(element[1])
            });

            const barChart = document.getElementById('barChart');
            const doughnutChart = document.getElementById('doughnutChart');
            barChart.innerHTML = '';
            doughnutChart.innerHTML = '';

            createChart(chart_labels,chart_data,'bar',barChart);
            createChart(chart_labels,chart_data,'doughnut',doughnutChart);
        }else{
            const errorData = await response.json();
            alert(`get group expenses failed: ${errorData.message || 'Unknown error'}`);
        }
    }catch (error){
        console.error("Error during group expenses: ", error);
        alert('An error occured. Please try again later.');
    };
}

function createChart(labels, data, type, element) {
    const canvas = document.createElement('canvas');
    canvas.id = 'myChart';
    element.appendChild(canvas);

    const ctx = canvas.getContext('2d');

    new Chart(ctx, {
        type: type, 
        data: {
            labels: labels, 
            datasets: [{
                label: '#PLN', 
                data: data, 
                borderWidth: 1
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            scales: {
                y: {
                    beginAtZero: true 
                }
            }
        }
    });
}

async function load_table(group_name){
    try{
        const response = await fetch(
            'https://rust-app-production.up.railway.app/expenses/group/' + group_name,{
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                }
        });
        if(response.ok){
            const data = await response.json();
            const tableContainer  = document.getElementById('table');
            tableContainer.innerHTML = '';
            const table = document.createElement("TABLE");
            
            // nagłówek
            const thead = table.createTHead();
            const headerRow = thead.insertRow();
            const headers = ["id", "person", "expense name", "cost"];  // Zmieniaj nazwy nagłówków
            headers.forEach(headerText => {
                const th = document.createElement("th");
                th.textContent = headerText;
                headerRow.appendChild(th);
            });

            const tbody = table.createTBody();
            data.forEach(element => {
                let row = tbody.insertRow(0);
                let cell0 = row.insertCell(0);
                let cell1 = row.insertCell(1);
                let cell2 = row.insertCell(2);
                let cell3 = row.insertCell(3);
                cell0.innerHTML = element[3];
                cell1.innerHTML = element[0];
                cell2.innerHTML = element[1];
                cell3.innerHTML = element[2];
            });
            tableContainer.appendChild(table);
        }else{
            const errorData = await response.json();
            alert(`get group expenses failed: ${errorData.message || 'Unknown error'}`);
        }
    }catch (error){
        console.error("Error during group expenses: ", error);
        alert('An error occured. Please try again later.');
    };
}



load_charts(group_name)
load_table(group_name)