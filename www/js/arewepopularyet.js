function layoutAndStyle() {
  var h = $(window).height();

  var bodySize = h / 500;
  if (bodySize <= 0.8) {
    bodySize = 0.8;
  } else if (bodySize >= 1.5) {
    bodySize = 1.5;
  }
  bodySize = bodySize <= 1.5 ? bodySize : 1.5;

  var answerSize = h / 100;
  answerSize = answerSize <= 2 ? 2 : answerSize;

  $('body').css({'font-size': bodySize + 'em'});
  $('.answer').css({'font-size': answerSize + 'em',
                   'margin-top': Math.round(h/8) + 'px'});
}

layoutAndStyle();

$(window).bind('resize', function(e) {
  layoutAndStyle();
});