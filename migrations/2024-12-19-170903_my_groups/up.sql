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
  <link rel="icon" type="image/x-icon" href="/static/favicon.ico">
</head>'),
('logged_in_belonging_groups_scripts', 
'<script src="/static/scripts/Auth/logOut.js"></script>
<script src="/static/scripts/CRUD/belonging_group_chart.js"></script>
<script src="https://cdn.jsdelivr.net/npm/chart.js"></script>');