document.addEventListener('DOMContentLoaded', () => {
    const form = document.getElementById("delete_expense_form");
    form.addEventListener('submit', async (event) =>{
        event.preventDefault();
        
        const formData = new FormData(form);
        const data = Object.fromEntries(formData.entries());

        try{
            const response = await fetch(
                'https://rust-app-production.up.railway.app/expense/' + data.expense_id,{
                    method: 'DELETE',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify(data),
            });

            if(response.ok){
                alert('delete expense succesful!');
                location.reload();
            } else{
                const errorData = await response.json();
                alert(`delete expense failed: ${errorData.message || 'Unknown error'}`);
            }
        } catch (error){
            console.error("Error during delete expense: ", error);
            alert('An error occured. Please try again later.');
        };
    });
})