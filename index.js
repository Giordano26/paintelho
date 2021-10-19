
var drawing = {
  points: [],
  rect: [],
  triangle:[]
};


  function _(selector){
    return document.querySelector(selector);
  }

  function setup(){
    let canvas = createCanvas(1250, 600);
    canvas.parent("canvas-wrapper");
    background(255);
  }

  function mouseDragged(){
    let type = FindBrushType();
    let size = parseInt(_("#pen-size").value);
    let color = _("#pen-color").value;
    fill(color);
    stroke(color);

    switch (type){
      case "pencil":
        line(pmouseX, pmouseY, mouseX, mouseY);
        drawing.points.push([pmouseX, pmouseY, mouseX, mouseY])
        break;
      case "brush":
        ellipse(mouseX, mouseY, size, size);
        break;
    }
  }

  function mousePressed() {
    let type = FindBrushType();
    let size = parseInt(_("#pen-size").value);
    console.log(size)

    switch (type){
      case "rectangle":
        rectMode(CENTER);
        rect(mouseX, mouseY, 30+(size *1.5), 50+(size *1.5));    
        drawing.rect.push([mouseX, mouseY, 50, 50])   
        break;
      case "triangle":
        triangle(mouseX + (size * 5), mouseY, mouseX - 170 , mouseY, mouseX-85+ (size * 2.5) , mouseY-100 - (size*5)) ;
        drawing.triangle.push([mouseX + (size * 5), mouseY, mouseX - 170 , mouseY, mouseX-85+ (size * 2.5) , mouseY-100 - (size*5)])   
        break;
    }

  }

  _("#reset-canvas").addEventListener("click", function(){
    background(255);
    drawing = {
      points: [],
      rect: [],
      triangle:[]
    }
  });
  _("#save-canvas").addEventListener("click",function(){
    saveCanvas(canvas, "sketch", "png");
  });
  _("#save-json").addEventListener("click",function(){
    var dataStr = "data:text/json;charset=utf-8," + encodeURIComponent(JSON.stringify(drawing));
    var dlAnchorElem = document.getElementById('downloadAnchorElem');
    dlAnchorElem.setAttribute("href", dataStr);
    dlAnchorElem.setAttribute("download", "backup.json");
    dlAnchorElem.click();

  });

  _("#load-json").addEventListener("click",function(){
    let file = document.querySelector('#file')
    if (!file.value.length) return;
    let reader = new FileReader();
    reader.onload = displayFile;
    reader.readAsText(file.files[0])
  });

  function displayFile (event){
    background(255);
    
    let str = event.target.result;
    let json = JSON.parse(str);

    for (let i = 0; i < json.points.length; i++){
      line(json.points[i][0], json.points[i][1], json.points[i][2], json.points[i][3]); 
     }

    for (let i = 0; i < json.rect.length; i++){
        rect(json.rect[i][0], json.rect[i][1], json.rect[i][2], json.rect[i][3]);   
     }

     for (let i = 0; i < json.triangle.length; i++){
      triangle(json.triangle[i][0], json.triangle[i][1], json.triangle[i][2], json.triangle[i][3], json.triangle[i][4],json.triangle[i][5]); 
     }
  }

  function FindBrushType(){
      switch (true) {
        case _("#pen-pencil").checked:
            return "pencil"
        case _("#pen-brush").checked:
            return "brush"
        case _("#triangle").checked:
            return "triangle"
        case _("#rectangle").checked:
              return "rectangle"
      }

  }
  
  