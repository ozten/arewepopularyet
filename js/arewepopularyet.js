var NUM_DAYS_PLOT = 7;
var SCALING_FACTOR = 1000000;

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
                   'margin-top': Math.round(h/15) + 'px'});
  $('.caption').css({'font-size': bodySize * 0.7 + 'em'});
}

layoutAndStyle();

$(window).bind('resize', function(e) {
  layoutAndStyle();
});


$.getJSON('data/daily_counts.json', function(data, status, jqxhr) {

    var keys = Object.keys(data);
    var dates = [];
    for(var i=0; i < keys.length; i++) {
        var key = keys[i];
        dates.push(Date.parse(key));
    }
    dates = dates.sort().slice(0 - NUM_DAYS_PLOT);

    var dataPoints = [[],[],[], []];
    for (var i=0; i < dates.length; i++) {

        var d = new Date(dates[i]);
        var key = d.getUTCFullYear() + '-' +
                  (d.getUTCMonth() < 11 ? '0' : '') +
                 (d.getUTCMonth() + 1) + '-' +
                 (d.getUTCDate() < 10 ? '0' : '') +
                  d.getUTCDate();

        var metrics = ['websites', 'idproviders', 'facebook', 'baseline'];
        for (var j=0; j < metrics.length; j++) {

            if (!data[key] ||
                ! data[key][metrics[j]]) {

                continue;
            }

            //dataPoints[j].push([1001545200000 - d.getTime(), data[key][metrics[j]]]);

            dataPoints[j].push([d.getTime(), data[key][metrics[j]]]);
        }

    }

    var plotOpts = {
      yaxis: {
        ticks: dataPoints[0].length + 1
      },
      xaxis: {
        mode: "time",
        timeformat: "%m/%d",
        minTickSize: [1, "day"]
      }
    };

    $.plot('.figures .websites', [{
      data:dataPoints[0]
    }], plotOpts);

    $.plot('.figures .idps', [{
      data:dataPoints[1]
    }], plotOpts);

    $.plot('.figures .facebook', [{
      mode: "time",
      timeformat: "%Y/%m/%d",
      data:dataPoints[2]
    }], plotOpts);

    $.plot('.figures .baseline', [{
      mode: "time",
      timeformat: "%Y/%m/%d",
      data:dataPoints[3]
    }], plotOpts);

    var baselineCount = dataPoints[3][dataPoints[3].length -1][1] - dataPoints[3][0][1];
    $('#new-repos').text(baselineCount);
    $('#scaling-factor').text(SCALING_FACTOR);
    function adoptionFactor(project, baseline) {

        return Math.round(project / baseline * SCALING_FACTOR);
    }
    $('.facts .website .factor').text(
        adoptionFactor(dataPoints[0][dataPoints[0].length -1][1] - dataPoints[0][0][1],
            baselineCount));

    $('.facts .facebook .factor').text(
        adoptionFactor(dataPoints[2][dataPoints[2].length -1][1] - dataPoints[2][0][1],
        baselineCount));
    $('.facts .baseline .factor').text(baselineCount);

});


$.getJSON('data/daily_repositories.json', function(data, status, jqxhr) {
  var keys = Object.keys(data);
  console.log(keys.join(','));
  keys.sort();
  console.log(keys.join(','));
  var today = keys[keys.length -1];
  $('.today').text(today);
  // TODO provide paginated access
  $('.yesterday, .tomorrow').hide();
  var liHtml = "";
  $(data[today].websites.adopters).each(function(i, repo){
    console.log(repo);
    liHtml += '<li><a href="https://github.com/' + repo + '">' + repo + '</a></li>';
  });
  $('.websites .adopters ul').append(liHtml);

  var liHtml = "";
  $(data[today].websites.defectors).each(function(i, repo){
    liHtml += '<li><a href="https://github.com/' + repo + '">' + repo + '</a></li>';
  });
  $('.websites .defectors ul').append(liHtml);

  var liHtml = "";
  $(data[today].idps.adopters).each(function(i, repo){
    console.log(repo);
    liHtml += '<li><a href="https://github.com/' + repo + '">' + repo + '</a></li>';
  });
  $('.idps .adopters ul').append(liHtml);

    var liHtml = "";
  $(data[today].idps.defectors).each(function(i, repo){
    liHtml += '<li><a href="https://github.com/' + repo + '">' + repo + '</a></li>';
  });
  $('.idps .defectors ul').append(liHtml);
});

