-- Your SQL goes here
-- Your SQL goes here
-- Dodanie rekordów do tabeli HTML
INSERT INTO "HTML" (name, element) VALUES
('logged_in_belonging_groups_head', 
'<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&display=swap" rel="stylesheet">
  <title>Roommates Organiser</title>
  <link rel="stylesheet" href="/static/css/base-styles.css">
  <link rel="stylesheet" href="/static/css/logo.css">
  <link rel="stylesheet" href="/static/css/genericButton.css">
  <link rel="stylesheet" href="/static/css/genericContent.css">
  <link rel="stylesheet" href="/static/css/drop-down.css">
  <link rel="stylesheet" href="/static/css/footer.css">
  <link rel="stylesheet" href="/static/css/Mygroup/groupButton.css">
  <link rel="icon" type="image/x-icon" href="/static/favicon.ico">
</head>'),
('logged_in_belonging_groups_specific_head', 
'<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <link href="https://fonts.googleapis.com/css2?family=Bebas+Neue&display=swap" rel="stylesheet">
  <title>Roommates Organiser</title>
  <link rel="stylesheet" href="/static/css/base-styles.css">
  <link rel="stylesheet" href="/static/css/logo.css">
  <link rel="stylesheet" href="/static/css/genericButton.css">
  <link rel="stylesheet" href="/static/css/genericContent.css">
  <link rel="stylesheet" href="/static/css/drop-down.css">
  <link rel="stylesheet" href="/static/css/footer.css">
  <link rel="stylesheet" href="/static/css/Mygroup/groupButton.css">
  <link rel="stylesheet" href="/static/css/Mygroup/GroupsExpensesCharts.css">
  <link rel="stylesheet" href="/static/css/Mygroup/table.css">
  <link rel="stylesheet" href="/static/css/authForm.css">
  <link rel="stylesheet" href="/static/css/ManageOwnegGroups.css/Formvisibility.css">
  <link rel="icon" type="image/x-icon" href="/static/favicon.ico">
</head>'),
('logged_in_belonging_groups_scripts', 
'<script src="/static/scripts/Auth/logOut.js"></script>
<script src="/static/scripts/Auth/session_checker.js"></script>
<script src="/static/scripts/CRUD/belonging_group_chart.js"></script>
<script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
<script src="/static/scripts/class_activator.js"></script>
<script src="/static/scripts/sectionActivator.js"></script>
<script src="/static/scripts/CRUD/add_expense_button.js"></script>
<script src="/static/scripts/CRUD/update_expense_button.js"></script>
<script src="/static/scripts/CRUD/delete_expense_button.js"></script>'
),
('logged_in_belonging_groups_CRUD_expenses',
'
<div class=crud_buttons_container>
<button id="add_expense" class="crud_notpicked groupButton color" onclick="update_class(''add_expense'', ''crud_picked'', ''crud_notpicked'' );  showSection(''add_expense_section'', ''Form'')" style="width:33.3%">add expense</button>
<button id="update_expense" class="crud_notpicked groupButton color" onclick="update_class(''update_expense'', ''crud_picked'', ''crud_notpicked'' ); showSection(''update_expense_section'', ''Form'')" style="width:33.3%">update expense</button>
<button id="delete_expense" class="crud_notpicked groupButton color" onclick="update_class(''delete_expense'', ''crud_picked'', ''crud_notpicked'' ); showSection(''delete_expense_section'', ''Form'')" style="width:33.3%">delete expense</button>
</div>

<section id="add_expense_section" class="content genericContent Form">
  <p>Provide details</p>
  <form class="authForm" id="add_expense_form">
    <div class="form-group">
      <label for="group_name">Group Name</label>
      <input type="text" id="groupp_name" name="group_name" required>

      <label for="expense_name">Expense Name</label>
      <input type="text" id="expense_name" name="name" required>

      <label for="expense_cost">Expense Cost</label>
      <input type="text" id="expense_cost" name="cost" required>
    </div>
    <button class="genericButton" type="submit">Add Expense</button>
  </form>
</section>

<section id="update_expense_section" class="content genericContent Form">
  <p>Provide details</p>
  <form class="authForm" id="update_expense_form">
    <div class="form-group">
      <label for="update_expense_name">Expense Name</label>
      <input type="text" id="update_expense_name" name="name" required>

      <label for="update_expense_cost">Expense Cost</label>
      <input type="text" id="update_expense_cost" name="cost" required>

      <label for="update_expense_id">Expense ID</label>
      <input type="text" id="update_expense_id" name="expense_id" type="number" step="1" required>
    </div>
    <button class="genericButton" type="submit">Update Expense</button>
  </form>
</section>


<section id="delete_expense_section" class="content genericContent Form">
  <p>Provide details</p>
  <form class="authForm" id="delete_expense_form">
    <div class="form-group">
      <label for="delete_expense_id">Expense ID</label>
      <input type="text" id="delete_expense_id" name="expense_id" required>
    </div>
    <button class="genericButton" type="submit">Delete Expense</button>
  </form>
</section>
');