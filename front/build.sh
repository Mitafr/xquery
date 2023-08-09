#!/bin/sh

yarn build

head -n -7 ./dist/index.html > tmp.txt && mv tmp.txt ./dist/index.html
echo "
<body>
  {% if authenticated %}
  <div id="app" data-authenticated="true" data-username="{{ username }}"></div>
  {% else %}
  <div id="app" data-authenticated="false"></div>
  {% endif %}
</body>

</html>" >> ./dist/index.html
