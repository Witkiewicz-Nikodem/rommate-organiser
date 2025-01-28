document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById("update_expense_form");
    form.addEventListener('submit', async (event) =>{
        event.preventDefault();
        
        const formData = new FormData(form);
        const data = Object.fromEntries(formData.entries());
        const expenseId = parseInt(data.expense_id, 10);
        data.expense_id = expenseId;

        console.log(JSON.stringify(data));

        try{
            const response = await fetch(
                'https://rust-app-production.up.railway.app/expense',{
                    method: 'PUT',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(data),
            });

            if(response.ok){
                alert('update expense succesful!');
                location.reload();
            } else{
                const errorData = await response.json();
                alert(`update expense failed: ${errorData.message || 'Unknown error'}`);
            }
        } catch (error){
            console.error("Error during update expense: ", error);
            alert('An error occured. Please try again later.');
        };
    });
})