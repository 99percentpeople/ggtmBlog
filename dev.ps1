start-process -FilePath "cargo" -ArgumentList ("run", "--bin", "blog-server") -NoNewWindow
start-process -FilePath "yarn" -ArgumentList ("workspace", "blog" , "dev") -NoNewWindow -Wait 