use crate::blast::types::BlastStatus;
use anyhow::{bail, Result};
use regex::Regex;

fn parse_qblast_info(text: &str, key: &str) -> Result<String> {
    let re = Regex::new(key)?;
    match re.find(text) {
        Some(m) => {
            let value = text
                .chars()
                // Not sure why I need this offset, but won't work without it
                .skip(m.start() + key.len() - 4)
                .take_while(|c| *c != '\n')
                .fold(String::new(), |mut s, c| {
                    s.push(c);
                    s
                });
            Ok(value)
        }
        None => bail!("Could not find query string"),
    }
}

pub fn parse_rid(text: &str) -> Result<String> {
    if let Ok(value) = parse_qblast_info(text, "    RID = ") {
        Ok(value)
    } else {
        bail!("No RID found in response");
    }
}

pub fn parse_rtoe(text: &str) -> Result<usize> {
    if let Ok(value) = parse_qblast_info(text, "    RTOE = ") {
        Ok(value.parse()?)
    } else {
        bail!("No RTOE found in response");
    }
}

pub fn parse_status(text: &str) -> Result<BlastStatus> {
    if let Ok(value) = parse_qblast_info(text, "      Status=") {
        BlastStatus::from_str(&value)
    } else {
        bail!("No status found in response");
    }
}

#[cfg(test)]
mod testing {
    use crate::blast::types::BlastStatus;

    use super::{parse_rid, parse_rtoe, parse_status};

    const EXAMPLE_STATUS_READY: &'static str = r##"
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en">
<head>
<meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
<meta name="jig" content="ncbitoggler"/>
<meta name="ncbitoggler" content="animation:'none'"/>
<title>NCBI Blast:</title>
<script type="text/javascript" src="/core/jig/1.15.2/js/jig.min.js             "></script>
<script type="text/javascript">    jQuery.getScript("/core/alerts/alerts.js", function() {
        galert(['div#header', 'body > *:nth-child(1)'])
    });</script>
<meta http-equiv="Pragma" content="no-cache">
<link rel="stylesheet" type="text/css" href="css/uswds.min.css" media="screen" />
<link rel="stylesheet"  type="text/css" href="https://www.ncbi.nlm.nih.gov/style-guide/static/nwds/css/nwds.css"/>
<link rel="stylesheet" href="css/headerNew.css?v=1"/>
<link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.5.0/css/all.css" crossorigin="anonymous"> <!-- Font Awesome icons -->
<link rel="stylesheet" type="text/css" href="css/footerNew.css?v=1" media="screen" />
<link rel="stylesheet" type="text/css" href="css/main.css" media="screen" />
<link rel="stylesheet" type="text/css" href="css/blastRes.css" media="screen" />
<link rel="stylesheet" type="text/css" href="css/print.css" media="print" />
<!--[if lte IE 6]>
<link rel="stylesheet" type="text/css" href="css/ie6_or_less.css" />
<![endif]-->
<script type="text/javascript" src="js/utils.js"></script>
<script type="text/javascript" src="js/results.js"></script>
</head>

<body id="type-a" class="noToggleCheck" >
<div id="wrap">
		<script>var useOfficialGovtHeader = true;</script>
<section class="usa-banner">
  <div class="usa-accordion">
    <header class="usa-banner-header">
      <div class="usa-grid usa-banner-inner">
        <img src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/favicons/favicon-57.png" alt="U.S. flag">
        <p>An official website of the United States government</p>
        <button class="usa-accordion-button usa-banner-button" aria-expanded="false" aria-controls="gov-banner-top">
          <span class="usa-banner-button-text">Here's how you know</span>
        </button>
      </div>
    </header>
    <div class="usa-banner-content usa-grid usa-accordion-content" id="gov-banner-top" aria-hidden="true">
      <div class="usa-banner-guidance-gov usa-width-one-half">
        <img class="usa-banner-icon usa-media_block-img" src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/icon-dot-gov.svg" alt="Dot gov">
        <div class="usa-media_block-body">
          <p>
            <strong>The .gov means it’s official.</strong>
            <br>
            Federal government websites often end in .gov or .mil. Before
            sharing sensitive information, make sure you’re on a federal
            government site.
          </p>
        </div>
      </div>
      <div class="usa-banner-guidance-ssl usa-width-one-half">
        <img class="usa-banner-icon usa-media_block-img" src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/icon-https.svg" alt="Https">
        <div class="usa-media_block-body">
          <p>
            <strong>The site is secure.</strong>
            <br>
            The <strong>https://</strong> ensures that you are connecting to the
            official website and that any information you provide is encrypted
            and transmitted securely.
          </p>
        </div>
      </div>
    </div>
  </div>
</section>
    	
<header class="ncbi-header" role="banner" data-section="Header">
<a class="usa-skipnav" href="#mainCont">Skip to main page content</a>
<div class="usa-grid">
    <div class="usa-width-one-whole">
        <div class="ncbi-header__logo">
                <a href="https://www.ncbi.nlm.nih.gov/" class="logo" aria-label="NCBI Logo" data-ga-action="click_image" data-ga-label="NIH NLM Logo">
                  <img src="https://www.ncbi.nlm.nih.gov/coreutils/nwds/img/logos/AgencyLogo.svg" alt="NIH NLM Logo">
                </a>
            </div>

        <div class="ncbi-header__account">
            <a id="account_login" href="https://www.ncbi.nlm.nih.gov/account/?back_url=https%3A%2F%2Fblast%2Encbi%2Enlm%2Enih%2Egov%2FBlast%2Ecgi%3FCMD%3DGet%26FORMAT%5FOBJECT%3DSearchInfo%26RID%3DNJVT5XRT013" class="usa-button header-button">Log in</a>
            <button id="account_info" class="header-button" aria-controls="account_popup">
                <span class="fa fa-user" aria-hidden="true"></span>
                <span class="username desktop-only" aria-hidden="true" id="uname_short"></span>
                <span class="sr-only">Show account info</span>
            </button>
        </div>

        <div class="ncbi-popup-anchor">
            <div class="ncbi-popup account-popup" id="account_popup" aria-hidden="true" role="dialog" aria-labelledby="account-popup-header">
                <div class="ncbi-popup-head">
                    <button class="ncbi-close-button"><span class="fa fa-window-close"></span><span class="usa-sr-only">Close</span></button>
                    <h4>Account</h4>
                </div>
                <div class="account-user-info">
                    Logged in as:<br>
                    <b><span class="username" id="uname_long">username</span></b>
                </div>
                <div class="account-links">
                    <ul class="usa-unstyled-list">
                        <li><a id="account_myncbi" href="https://www.ncbi.nlm.nih.gov/myncbi/">Dashboard</a> <span class="ncbi-text-small-light">(My NCBI)</span></li>
                        <li><a id="account_pubs" href="https://www.ncbi.nlm.nih.gov/myncbi/collections/bibliography/">Publications</a> <span class="ncbi-text-small-light">(My Bibliography)</span></li>
                        <li><a id="account_settings" href="https://www.ncbi.nlm.nih.gov/account/settings/">Account settings</a></li>
                        <li><a id="account_logout" href="https://www.ncbi.nlm.nih.gov/account/signout/?back_url=https%3A%2F%2Fblast%2Encbi%2Enlm%2Enih%2Egov%2FBlast%2Ecgi%3FCMD%3DGet%26FORMAT%5FOBJECT%3DSearchInfo%26RID%3DNJVT5XRT013">Log out</a></li>
                    </ul>
                </div>
            </div>
        </div>

    </div>
</div>
</header>
<div role="navigation" aria-label="access keys">
<a id="nws_header_accesskey_0" href="https://www.ncbi.nlm.nih.gov/guide/browsers/#ncbi_accesskeys" class="usa-sr-only" accesskey="0" tabindex="-1">Access keys</a>
<a id="nws_header_accesskey_1" href="https://www.ncbi.nlm.nih.gov" class="usa-sr-only" accesskey="1" tabindex="-1">NCBI Homepage</a>
<a id="nws_header_accesskey_2" href="/myncbi/" class="set-base-url usa-sr-only" accesskey="2" tabindex="-1">MyNCBI Homepage</a>
<a id="nws_header_accesskey_3" href="#maincontent" class="usa-sr-only" accesskey="3" tabindex="-1">Main Content</a>
<a id="nws_header_accesskey_4" href="#" class="usa-sr-only" accesskey="4" tabindex="-1">Main Navigation</a>
</div>
<nav class="ncbi-topnav" id="navcontent">
    <div class="usa-grid">
        <a class="ncbi-topnav-root" href="Blast.cgi">BLAST <sup>&reg;</sup></a> <span id="brc"><span class="brcrmbsign">&raquo;</span> <a href="Blast.cgi?PAGE=Nucleotides&amp;PROGRAM=blastn&amp;PAGE_TYPE=BlastSearch&amp;BLAST_SPEC=">blastn suite</a> <span class="brcrmbsign">&raquo;</span> RID-NJVT5XRT013</span>
        <ul class="rf ncbi-topnav-list" id="topnav-list">
            <li class="first "><a href="Blast.cgi?CMD=Web&amp;PAGE_TYPE=BlastHome" title="BLAST Home">Home</a></li>
            <li class="recent "><a href="Blast.cgi?CMD=GetSaved&amp;RECENT_RESULTS=on" title="Unexpired BLAST jobs">Recent Results</a></li>                
            <li class="saved "><a href="Blast.cgi?CMD=GetSaved" title="Saved sets of BLAST search parameters">Saved Strategies</a></li>
            <li  class= "last documentation "> <a href="../doc/blast-help/" title="BLAST documentation">Help</a></li>                            
        </ul>
    </div>
</nav>



        <div id="content-wrap">

                <div class="pageTitle">                            
                   Format Request Status                   
                </div>
		<div class="inlineDiv resHeader">				   
				   <a  id="frmPage"  class="WAITING" href="#" submitForm="reformat">[Formatting options] </a>                   
                </div>
                <h2 id="jtitle" >Job Title: </h2>
								
                <div id="content">                
                <!--<ul id="msg" class="msg"><li class=""><p class=""></p><p class=""></p><p class=""></p></ul> -->
                <ul id="msg" class="msg"><li class=""></li></ul>
                <p><!--
                QBlastInfoBegin
	                Status=READY
                QBlastInfoEnd
                --></p> 
                               
                <SCRIPT LANGUAGE="JavaScript"><!--
                    var tm = "2000";
                    if (tm != "") {                    
                        setTimeout('document.forms[0].submit();',tm);
                    }
                //--></SCRIPT>                                
                <table id="statInfo" class="WAITING">
                <tr><td>Request ID</td><td> <b>NJVT5XRT013</b></td></tr>
                <tr class="odd"><td>Status</td><td>Searching</td></tr>
                <tr><td>Submitted at</td><td>Wed Oct 26 13:32:21 2022</td></tr>
                <tr class="odd"><td>Current time</td><td>Wed Oct 26 13:33:06 2022</td></tr>
                <tr><td>Time since submission</td><td>00:00:45</td></tr>
                </table>
                <p class="WAITING">This page will be automatically updated in <b>2</b> seconds</p>       
                <form action="Blast.cgi" enctype="application/x-www-form-urlencoded" method="POST" id="results">
                <input name="FORMAT_OBJECT" type="hidden" value="SearchInfo"><input name="RID" type="hidden" value="NJVT5XRT013"><input name="SEARCH_DB_STATUS" type="hidden" value="21"><input name="USER_TYPE" type="hidden" value="2"><input name="_PGR" type="hidden" value="0">                
                <input name="_PGR" type="hidden" value="0" >
                <input name="CMD" type="hidden" value="Get">
               
                </form>
                                
				</div><!-- /#content -->
				<form action="Blast.cgi" enctype="application/x-www-form-urlencoded"  method="post" name="reformat" id="reformat">				
				   <input name="QUERY_INFO" type="hidden" value="" />    				
				   <input name="ENTREZ_QUERY" type="hidden" value="" />
                   <input name="CDD_RID" type="hidden" value="" />
                   <input name="CDD_SEARCH_STATE" type="hidden" value="" />
                   <input name="RID" type="hidden" value="NJVT5XRT013" />
				   <input name="STEP_NUMBER" type="hidden" value="" />
				   <input name="CMD" type="hidden" value="Web"/>				
				   <input NAME="PAGE_TYPE" type="hidden"  value="BlastFormatting"/>
				
				   <!-- TO DO: test all of those changes -->				   	
				   <!-- Psi blast params  PSI_BLAST_PARAMS - commented- using forms[0] from fromatter> -->
				   <!-- Current Formatting options FORMATTING_OPTIONS- commented- using forms[0] from fromatter> -->
				   <!-- Current Search options CURR_SAVED_OPTIONS - commented- using forms[0] from fromatter> -->
                 </form>				
        </div><!-- /#content-wrap -->

         <footer>
      <section class="icon-section">
        <div id="icon-section-header" class="icon-section_header">Follow NCBI</div>
        <div class="grid-container container">
          <div class="icon-section_container">
            <a class="footer-icon" id="footer_twitter" href="https://twitter.com/ncbi" aria-label="Twitter"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <defs>
                  <style>
                    .cls-11 {
                      fill: #737373;
                    }
                  </style>
                </defs>
                <title>Twitter</title>
                <path class="cls-11" d="M250.11,105.48c-7,3.14-13,3.25-19.27.14,8.12-4.86,8.49-8.27,11.43-17.46a78.8,78.8,0,0,1-25,9.55,39.35,39.35,0,0,0-67,35.85,111.6,111.6,0,0,1-81-41.08A39.37,39.37,0,0,0,81.47,145a39.08,39.08,0,0,1-17.8-4.92c0,.17,0,.33,0,.5a39.32,39.32,0,0,0,31.53,38.54,39.26,39.26,0,0,1-17.75.68,39.37,39.37,0,0,0,36.72,27.3A79.07,79.07,0,0,1,56,223.34,111.31,111.31,0,0,0,116.22,241c72.3,0,111.83-59.9,111.83-111.84,0-1.71,0-3.4-.1-5.09C235.62,118.54,244.84,113.37,250.11,105.48Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_facebook" href="https://www.facebook.com/ncbi.nlm" aria-label="Facebook"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <title>Facebook</title>
                <path class="cls-11" d="M210.5,115.12H171.74V97.82c0-8.14,5.39-10,9.19-10h27.14V52l-39.32-.12c-35.66,0-42.42,26.68-42.42,43.77v19.48H99.09v36.32h27.24v109h45.41v-109h35Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_linkedin" href="https://www.linkedin.com/company/ncbinlm" aria-label="LinkedIn"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <title>LinkedIn</title>
                <path class="cls-11" d="M101.64,243.37H57.79v-114h43.85Zm-22-131.54h-.26c-13.25,0-21.82-10.36-21.82-21.76,0-11.65,8.84-21.15,22.33-21.15S101.7,78.72,102,90.38C102,101.77,93.4,111.83,79.63,111.83Zm100.93,52.61A17.54,17.54,0,0,0,163,182v61.39H119.18s.51-105.23,0-114H163v13a54.33,54.33,0,0,1,34.54-12.66c26,0,44.39,18.8,44.39,55.29v58.35H198.1V182A17.54,17.54,0,0,0,180.56,164.44Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_github" href="https://github.com/ncbi" aria-label="GitHub"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <defs>
                  <style>
                    .cls-11,
                    .cls-12 {
                      fill: #737373;
                    }

                    .cls-11 {
                      fill-rule: evenodd;
                    }
                  </style>
                </defs>
                <title>GitHub</title>
                <path class="cls-11" d="M151.36,47.28a105.76,105.76,0,0,0-33.43,206.1c5.28,1,7.22-2.3,7.22-5.09,0-2.52-.09-10.85-.14-19.69-29.42,6.4-35.63-12.48-35.63-12.48-4.81-12.22-11.74-15.47-11.74-15.47-9.59-6.56.73-6.43.73-6.43,10.61.75,16.21,10.9,16.21,10.9,9.43,16.17,24.73,11.49,30.77,8.79,1-6.83,3.69-11.5,6.71-14.14C108.57,197.1,83.88,188,83.88,147.51a40.92,40.92,0,0,1,10.9-28.39c-1.1-2.66-4.72-13.42,1-28,0,0,8.88-2.84,29.09,10.84a100.26,100.26,0,0,1,53,0C198,88.3,206.9,91.14,206.9,91.14c5.76,14.56,2.14,25.32,1,28a40.87,40.87,0,0,1,10.89,28.39c0,40.62-24.74,49.56-48.29,52.18,3.79,3.28,7.17,9.71,7.17,19.58,0,14.15-.12,25.54-.12,29,0,2.82,1.9,6.11,7.26,5.07A105.76,105.76,0,0,0,151.36,47.28Z">
                </path>
                <path class="cls-12" d="M85.66,199.12c-.23.52-1.06.68-1.81.32s-1.2-1.06-.95-1.59,1.06-.69,1.82-.33,1.21,1.07.94,1.6Zm-1.3-1">
                </path>
                <path class="cls-12" d="M90,203.89c-.51.47-1.49.25-2.16-.49a1.61,1.61,0,0,1-.31-2.19c.52-.47,1.47-.25,2.17.49s.82,1.72.3,2.19Zm-1-1.08">
                </path>
                <path class="cls-12" d="M94.12,210c-.65.46-1.71,0-2.37-.91s-.64-2.07,0-2.52,1.7,0,2.36.89.65,2.08,0,2.54Zm0,0"></path>
                <path class="cls-12" d="M99.83,215.87c-.58.64-1.82.47-2.72-.41s-1.18-2.06-.6-2.7,1.83-.46,2.74.41,1.2,2.07.58,2.7Zm0,0">
                </path>
                <path class="cls-12" d="M107.71,219.29c-.26.82-1.45,1.2-2.64.85s-2-1.34-1.74-2.17,1.44-1.23,2.65-.85,2,1.32,1.73,2.17Zm0,0">
                </path>
                <path class="cls-12" d="M116.36,219.92c0,.87-1,1.59-2.24,1.61s-2.29-.68-2.3-1.54,1-1.59,2.26-1.61,2.28.67,2.28,1.54Zm0,0">
                </path>
                <path class="cls-12" d="M124.42,218.55c.15.85-.73,1.72-2,1.95s-2.37-.3-2.52-1.14.73-1.75,2-2,2.37.29,2.53,1.16Zm0,0"></path>
              </svg></a>
            <a class="footer-icon" id="footer_blog" href="https://ncbiinsights.ncbi.nlm.nih.gov/" aria-label="Blog">
              <svg id="Layer_1" data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 40 40"><defs><style>.cls-1{fill:#737373;}</style></defs><path class="cls-1" d="M14,30a4,4,0,1,1-4-4,4,4,0,0,1,4,4Zm11,3A19,19,0,0,0,7.05,15a1,1,0,0,0-1,1v3a1,1,0,0,0,.93,1A14,14,0,0,1,20,33.07,1,1,0,0,0,21,34h3a1,1,0,0,0,1-1Zm9,0A28,28,0,0,0,7,6,1,1,0,0,0,6,7v3a1,1,0,0,0,1,1A23,23,0,0,1,29,33a1,1,0,0,0,1,1h3A1,1,0,0,0,34,33Z"></path></svg>
            </a>
          </div>
        </div>
      </section>

      <section class="container-fluid bg-primary">
        <div class="container pt-5">
          <div class="row mt-3">
            <div class="col-lg-3 col-12">
              <p><a class="text-white" href="https://www.nlm.nih.gov/socialmedia/index.html">Connect with NLM</a></p>
              <ul class="list-inline social_media">
                <li class="list-inline-item"><a href="https://twitter.com/NLM_NIH" aria-label="Twitter" target="_blank" rel="noopener noreferrer"><svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <style type="text/css">
                        .st20 {
                          fill: #FFFFFF;
                        }

                        .st30 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }
                      </style>
                      <title>SM-Twitter</title>
                      <g>
                        <g>
                          <g>
                            <path class="st20" d="M192.9,88.1c-5,2.2-9.2,2.3-13.6,0.1c5.7-3.4,6-5.8,8.1-12.3c-5.4,3.2-11.4,5.5-17.6,6.7
                                                c-10.5-11.2-28.1-11.7-39.2-1.2c-7.2,6.8-10.2,16.9-8,26.5c-22.3-1.1-43.1-11.7-57.2-29C58,91.6,61.8,107.9,74,116
                                                c-4.4-0.1-8.7-1.3-12.6-3.4c0,0.1,0,0.2,0,0.4c0,13.2,9.3,24.6,22.3,27.2c-4.1,1.1-8.4,1.3-12.5,0.5c3.6,11.3,14,19,25.9,19.3
                                                c-11.6,9.1-26.4,13.2-41.1,11.5c12.7,8.1,27.4,12.5,42.5,12.5c51,0,78.9-42.2,78.9-78.9c0-1.2,0-2.4-0.1-3.6
                                                C182.7,97.4,189.2,93.7,192.9,88.1z"></path>
                          </g>
                        </g>
                        <circle class="st30" cx="124.4" cy="128.8" r="108.2"></circle>
                      </g>
                    </svg></a></li>
                <li class="list-inline-item"><a href="https://www.facebook.com/nationallibraryofmedicine" aria-label="Facebook" rel="noopener noreferrer" target="_blank">
                    <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <style type="text/css">
                        .st10 {
                          fill: #FFFFFF;
                        }

                        .st110 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }
                      </style>
                      <title>SM-Facebook</title>
                      <g>
                        <g>
                          <path class="st10" d="M159,99.1h-24V88.4c0-5,3.3-6.2,5.7-6.2h16.8V60l-24.4-0.1c-22.1,0-26.2,16.5-26.2,27.1v12.1H90v22.5h16.9
                                                      v67.5H135v-67.5h21.7L159,99.1z"></path>
                        </g>
                      </g>
                      <circle class="st110" cx="123.6" cy="123.2" r="108.2"></circle>
                    </svg>
                  </a></li>
                <li class="list-inline-item"><a href="https://www.youtube.com/user/NLMNIH" aria-label="Youtube" target="_blank" rel="noopener noreferrer"><svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <title>SM-Youtube</title>
                      <style type="text/css">
                        .st4 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }

                        .st5 {
                          fill: #FFFFFF;
                        }
                      </style>
                      <circle class="st4" cx="124.2" cy="123.4" r="108.2"></circle>
                      <g transform="translate(0,-952.36218)">
                        <path class="st5" d="M88.4,1037.4c-10.4,0-18.7,8.3-18.7,18.7v40.1c0,10.4,8.3,18.7,18.7,18.7h72.1c10.4,0,18.7-8.3,18.7-18.7
                                            v-40.1c0-10.4-8.3-18.7-18.7-18.7H88.4z M115.2,1058.8l29.4,17.4l-29.4,17.4V1058.8z"></path>
                      </g>
                    </svg></a></li>
              </ul>
            </div>
            <div class="col-lg-3 col-12">
              <p class="address_footer text-white">National Library of Medicine<br>
                <a href="https://www.google.com/maps/place/8600+Rockville+Pike,+Bethesda,+MD+20894/@38.9959508,-77.101021,17z/data=!3m1!4b1!4m5!3m4!1s0x89b7c95e25765ddb:0x19156f88b27635b8!8m2!3d38.9959508!4d-77.0988323" class="text-white" target="_blank" rel="noopener noreferrer">8600 Rockville Pike<br>
                  Bethesda, MD 20894</a></p>
            </div>
            <div class="col-lg-3 col-12 centered-lg">
              <p><a href="https://www.nlm.nih.gov/web_policies.html" class="text-white">Web Policies</a><br>
                <a href="https://www.nih.gov/institutes-nih/nih-office-director/office-communications-public-liaison/freedom-information-act-office" class="text-white">FOIA</a><br>
                <a href="https://www.hhs.gov/vulnerability-disclosure-policy/index.html" class="text-white" id="vdp">HHS Vulnerability Disclosure</a></p>
            </div>
            <div class="col-lg-3 col-12 centered-lg">
              <p><a class="supportLink text-white" href="https://support.nlm.nih.gov/">Help</a><br>
                <a href="https://www.nlm.nih.gov/accessibility.html" class="text-white">Accessibility</a><br>
                <a href="https://www.nlm.nih.gov/careers/careers.html" class="text-white">Careers</a></p>
            </div>
          </div>
          <div class="row">
            <div class="col-lg-12 centered-lg">
              <nav class="bottom-links">
                <ul class="mt-3">
                  <li>
                    <a class="text-white" href="//www.nlm.nih.gov/">NLM</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.nih.gov/">NIH</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.hhs.gov/">HHS</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.usa.gov/">USA.gov</a>
                  </li>
                </ul>
              </nav>
            </div>
          </div>
        </div>
      </section>
    </footer>
<script type="text/javascript" src="js/nwds.js"></script>
<script type="text/javascript" src="js/ncbipopup.js"></script>
<script type="text/javascript" src="js/headerNew.js"></script>
<script src="js/uswds.min.js"></script>
        <script type="text/javascript" src="/portal/portal3rc.fcgi/rlib/js/InstrumentOmnitureBaseJS/InstrumentNCBIBaseJS/InstrumentPageStarterJS.js"></script>
<!--
<script type="text/javascript" src="://www.ncbi.nlm.nih.gov/portal/portal3rc.fcgi/supportedbrowsers/js/nonportalbc_min.js"></script>
<script type="text/javascript">$("#browsers_ajax").browserCheck();</script>
-->
   </div><!--/#wrap-->
</body>

</html>"##;
    const EXAMPLE_STATUS_WAITING: &'static str = r##"
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en">
<head>
<meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
<meta name="jig" content="ncbitoggler"/>
<meta name="ncbitoggler" content="animation:'none'"/>
<title>NCBI Blast:</title>
<script type="text/javascript" src="/core/jig/1.15.2/js/jig.min.js             "></script>
<script type="text/javascript">    jQuery.getScript("/core/alerts/alerts.js", function() {
        galert(['div#header', 'body > *:nth-child(1)'])
    });</script>
<meta http-equiv="Pragma" content="no-cache">
<link rel="stylesheet" type="text/css" href="css/uswds.min.css" media="screen" />
<link rel="stylesheet"  type="text/css" href="https://www.ncbi.nlm.nih.gov/style-guide/static/nwds/css/nwds.css"/>
<link rel="stylesheet" href="css/headerNew.css?v=1"/>
<link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.5.0/css/all.css" crossorigin="anonymous"> <!-- Font Awesome icons -->
<link rel="stylesheet" type="text/css" href="css/footerNew.css?v=1" media="screen" />
<link rel="stylesheet" type="text/css" href="css/main.css" media="screen" />
<link rel="stylesheet" type="text/css" href="css/blastRes.css" media="screen" />
<link rel="stylesheet" type="text/css" href="css/print.css" media="print" />
<!--[if lte IE 6]>
<link rel="stylesheet" type="text/css" href="css/ie6_or_less.css" />
<![endif]-->
<script type="text/javascript" src="js/utils.js"></script>
<script type="text/javascript" src="js/results.js"></script>
</head>

<body id="type-a" class="noToggleCheck" >
<div id="wrap">
		<script>var useOfficialGovtHeader = true;</script>
<section class="usa-banner">
  <div class="usa-accordion">
    <header class="usa-banner-header">
      <div class="usa-grid usa-banner-inner">
        <img src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/favicons/favicon-57.png" alt="U.S. flag">
        <p>An official website of the United States government</p>
        <button class="usa-accordion-button usa-banner-button" aria-expanded="false" aria-controls="gov-banner-top">
          <span class="usa-banner-button-text">Here's how you know</span>
        </button>
      </div>
    </header>
    <div class="usa-banner-content usa-grid usa-accordion-content" id="gov-banner-top" aria-hidden="true">
      <div class="usa-banner-guidance-gov usa-width-one-half">
        <img class="usa-banner-icon usa-media_block-img" src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/icon-dot-gov.svg" alt="Dot gov">
        <div class="usa-media_block-body">
          <p>
            <strong>The .gov means it’s official.</strong>
            <br>
            Federal government websites often end in .gov or .mil. Before
            sharing sensitive information, make sure you’re on a federal
            government site.
          </p>
        </div>
      </div>
      <div class="usa-banner-guidance-ssl usa-width-one-half">
        <img class="usa-banner-icon usa-media_block-img" src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/icon-https.svg" alt="Https">
        <div class="usa-media_block-body">
          <p>
            <strong>The site is secure.</strong>
            <br>
            The <strong>https://</strong> ensures that you are connecting to the
            official website and that any information you provide is encrypted
            and transmitted securely.
          </p>
        </div>
      </div>
    </div>
  </div>
</section>
    	
<header class="ncbi-header" role="banner" data-section="Header">
<a class="usa-skipnav" href="#mainCont">Skip to main page content</a>
<div class="usa-grid">
    <div class="usa-width-one-whole">
        <div class="ncbi-header__logo">
                <a href="https://www.ncbi.nlm.nih.gov/" class="logo" aria-label="NCBI Logo" data-ga-action="click_image" data-ga-label="NIH NLM Logo">
                  <img src="https://www.ncbi.nlm.nih.gov/coreutils/nwds/img/logos/AgencyLogo.svg" alt="NIH NLM Logo">
                </a>
            </div>

        <div class="ncbi-header__account">
            <a id="account_login" href="https://www.ncbi.nlm.nih.gov/account/?back_url=https%3A%2F%2Fblast%2Encbi%2Enlm%2Enih%2Egov%2FBlast%2Ecgi%3FCMD%3DGet%26FORMAT%5FOBJECT%3DSearchInfo%26RID%3DNJVT5XRT013" class="usa-button header-button">Log in</a>
            <button id="account_info" class="header-button" aria-controls="account_popup">
                <span class="fa fa-user" aria-hidden="true"></span>
                <span class="username desktop-only" aria-hidden="true" id="uname_short"></span>
                <span class="sr-only">Show account info</span>
            </button>
        </div>

        <div class="ncbi-popup-anchor">
            <div class="ncbi-popup account-popup" id="account_popup" aria-hidden="true" role="dialog" aria-labelledby="account-popup-header">
                <div class="ncbi-popup-head">
                    <button class="ncbi-close-button"><span class="fa fa-window-close"></span><span class="usa-sr-only">Close</span></button>
                    <h4>Account</h4>
                </div>
                <div class="account-user-info">
                    Logged in as:<br>
                    <b><span class="username" id="uname_long">username</span></b>
                </div>
                <div class="account-links">
                    <ul class="usa-unstyled-list">
                        <li><a id="account_myncbi" href="https://www.ncbi.nlm.nih.gov/myncbi/">Dashboard</a> <span class="ncbi-text-small-light">(My NCBI)</span></li>
                        <li><a id="account_pubs" href="https://www.ncbi.nlm.nih.gov/myncbi/collections/bibliography/">Publications</a> <span class="ncbi-text-small-light">(My Bibliography)</span></li>
                        <li><a id="account_settings" href="https://www.ncbi.nlm.nih.gov/account/settings/">Account settings</a></li>
                        <li><a id="account_logout" href="https://www.ncbi.nlm.nih.gov/account/signout/?back_url=https%3A%2F%2Fblast%2Encbi%2Enlm%2Enih%2Egov%2FBlast%2Ecgi%3FCMD%3DGet%26FORMAT%5FOBJECT%3DSearchInfo%26RID%3DNJVT5XRT013">Log out</a></li>
                    </ul>
                </div>
            </div>
        </div>

    </div>
</div>
</header>
<div role="navigation" aria-label="access keys">
<a id="nws_header_accesskey_0" href="https://www.ncbi.nlm.nih.gov/guide/browsers/#ncbi_accesskeys" class="usa-sr-only" accesskey="0" tabindex="-1">Access keys</a>
<a id="nws_header_accesskey_1" href="https://www.ncbi.nlm.nih.gov" class="usa-sr-only" accesskey="1" tabindex="-1">NCBI Homepage</a>
<a id="nws_header_accesskey_2" href="/myncbi/" class="set-base-url usa-sr-only" accesskey="2" tabindex="-1">MyNCBI Homepage</a>
<a id="nws_header_accesskey_3" href="#maincontent" class="usa-sr-only" accesskey="3" tabindex="-1">Main Content</a>
<a id="nws_header_accesskey_4" href="#" class="usa-sr-only" accesskey="4" tabindex="-1">Main Navigation</a>
</div>
<nav class="ncbi-topnav" id="navcontent">
    <div class="usa-grid">
        <a class="ncbi-topnav-root" href="Blast.cgi">BLAST <sup>&reg;</sup></a> <span id="brc"><span class="brcrmbsign">&raquo;</span> <a href="Blast.cgi?PAGE=Nucleotides&amp;PROGRAM=blastn&amp;PAGE_TYPE=BlastSearch&amp;BLAST_SPEC=">blastn suite</a> <span class="brcrmbsign">&raquo;</span> RID-NJVT5XRT013</span>
        <ul class="rf ncbi-topnav-list" id="topnav-list">
            <li class="first "><a href="Blast.cgi?CMD=Web&amp;PAGE_TYPE=BlastHome" title="BLAST Home">Home</a></li>
            <li class="recent "><a href="Blast.cgi?CMD=GetSaved&amp;RECENT_RESULTS=on" title="Unexpired BLAST jobs">Recent Results</a></li>                
            <li class="saved "><a href="Blast.cgi?CMD=GetSaved" title="Saved sets of BLAST search parameters">Saved Strategies</a></li>
            <li  class= "last documentation "> <a href="../doc/blast-help/" title="BLAST documentation">Help</a></li>                            
        </ul>
    </div>
</nav>



        <div id="content-wrap">

                <div class="pageTitle">                            
                   Format Request Status                   
                </div>
		<div class="inlineDiv resHeader">				   
				   <a  id="frmPage"  class="WAITING" href="#" submitForm="reformat">[Formatting options] </a>                   
                </div>
                <h2 id="jtitle" >Job Title: </h2>
								
                <div id="content">                
                <!--<ul id="msg" class="msg"><li class=""><p class=""></p><p class=""></p><p class=""></p></ul> -->
                <ul id="msg" class="msg"><li class=""></li></ul>
                <p><!--
                QBlastInfoBegin
	                Status=WAITING
                QBlastInfoEnd
                --></p> 
                               
                <SCRIPT LANGUAGE="JavaScript"><!--
                    var tm = "2000";
                    if (tm != "") {                    
                        setTimeout('document.forms[0].submit();',tm);
                    }
                //--></SCRIPT>                                
                <table id="statInfo" class="WAITING">
                <tr><td>Request ID</td><td> <b>NJVT5XRT013</b></td></tr>
                <tr class="odd"><td>Status</td><td>Searching</td></tr>
                <tr><td>Submitted at</td><td>Wed Oct 26 13:32:21 2022</td></tr>
                <tr class="odd"><td>Current time</td><td>Wed Oct 26 13:33:06 2022</td></tr>
                <tr><td>Time since submission</td><td>00:00:45</td></tr>
                </table>
                <p class="WAITING">This page will be automatically updated in <b>2</b> seconds</p>       
                <form action="Blast.cgi" enctype="application/x-www-form-urlencoded" method="POST" id="results">
                <input name="FORMAT_OBJECT" type="hidden" value="SearchInfo"><input name="RID" type="hidden" value="NJVT5XRT013"><input name="SEARCH_DB_STATUS" type="hidden" value="21"><input name="USER_TYPE" type="hidden" value="2"><input name="_PGR" type="hidden" value="0">                
                <input name="_PGR" type="hidden" value="0" >
                <input name="CMD" type="hidden" value="Get">
               
                </form>
                                
				</div><!-- /#content -->
				<form action="Blast.cgi" enctype="application/x-www-form-urlencoded"  method="post" name="reformat" id="reformat">				
				   <input name="QUERY_INFO" type="hidden" value="" />    				
				   <input name="ENTREZ_QUERY" type="hidden" value="" />
                   <input name="CDD_RID" type="hidden" value="" />
                   <input name="CDD_SEARCH_STATE" type="hidden" value="" />
                   <input name="RID" type="hidden" value="NJVT5XRT013" />
				   <input name="STEP_NUMBER" type="hidden" value="" />
				   <input name="CMD" type="hidden" value="Web"/>				
				   <input NAME="PAGE_TYPE" type="hidden"  value="BlastFormatting"/>
				
				   <!-- TO DO: test all of those changes -->				   	
				   <!-- Psi blast params  PSI_BLAST_PARAMS - commented- using forms[0] from fromatter> -->
				   <!-- Current Formatting options FORMATTING_OPTIONS- commented- using forms[0] from fromatter> -->
				   <!-- Current Search options CURR_SAVED_OPTIONS - commented- using forms[0] from fromatter> -->
                 </form>				
        </div><!-- /#content-wrap -->

         <footer>
      <section class="icon-section">
        <div id="icon-section-header" class="icon-section_header">Follow NCBI</div>
        <div class="grid-container container">
          <div class="icon-section_container">
            <a class="footer-icon" id="footer_twitter" href="https://twitter.com/ncbi" aria-label="Twitter"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <defs>
                  <style>
                    .cls-11 {
                      fill: #737373;
                    }
                  </style>
                </defs>
                <title>Twitter</title>
                <path class="cls-11" d="M250.11,105.48c-7,3.14-13,3.25-19.27.14,8.12-4.86,8.49-8.27,11.43-17.46a78.8,78.8,0,0,1-25,9.55,39.35,39.35,0,0,0-67,35.85,111.6,111.6,0,0,1-81-41.08A39.37,39.37,0,0,0,81.47,145a39.08,39.08,0,0,1-17.8-4.92c0,.17,0,.33,0,.5a39.32,39.32,0,0,0,31.53,38.54,39.26,39.26,0,0,1-17.75.68,39.37,39.37,0,0,0,36.72,27.3A79.07,79.07,0,0,1,56,223.34,111.31,111.31,0,0,0,116.22,241c72.3,0,111.83-59.9,111.83-111.84,0-1.71,0-3.4-.1-5.09C235.62,118.54,244.84,113.37,250.11,105.48Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_facebook" href="https://www.facebook.com/ncbi.nlm" aria-label="Facebook"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <title>Facebook</title>
                <path class="cls-11" d="M210.5,115.12H171.74V97.82c0-8.14,5.39-10,9.19-10h27.14V52l-39.32-.12c-35.66,0-42.42,26.68-42.42,43.77v19.48H99.09v36.32h27.24v109h45.41v-109h35Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_linkedin" href="https://www.linkedin.com/company/ncbinlm" aria-label="LinkedIn"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <title>LinkedIn</title>
                <path class="cls-11" d="M101.64,243.37H57.79v-114h43.85Zm-22-131.54h-.26c-13.25,0-21.82-10.36-21.82-21.76,0-11.65,8.84-21.15,22.33-21.15S101.7,78.72,102,90.38C102,101.77,93.4,111.83,79.63,111.83Zm100.93,52.61A17.54,17.54,0,0,0,163,182v61.39H119.18s.51-105.23,0-114H163v13a54.33,54.33,0,0,1,34.54-12.66c26,0,44.39,18.8,44.39,55.29v58.35H198.1V182A17.54,17.54,0,0,0,180.56,164.44Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_github" href="https://github.com/ncbi" aria-label="GitHub"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <defs>
                  <style>
                    .cls-11,
                    .cls-12 {
                      fill: #737373;
                    }

                    .cls-11 {
                      fill-rule: evenodd;
                    }
                  </style>
                </defs>
                <title>GitHub</title>
                <path class="cls-11" d="M151.36,47.28a105.76,105.76,0,0,0-33.43,206.1c5.28,1,7.22-2.3,7.22-5.09,0-2.52-.09-10.85-.14-19.69-29.42,6.4-35.63-12.48-35.63-12.48-4.81-12.22-11.74-15.47-11.74-15.47-9.59-6.56.73-6.43.73-6.43,10.61.75,16.21,10.9,16.21,10.9,9.43,16.17,24.73,11.49,30.77,8.79,1-6.83,3.69-11.5,6.71-14.14C108.57,197.1,83.88,188,83.88,147.51a40.92,40.92,0,0,1,10.9-28.39c-1.1-2.66-4.72-13.42,1-28,0,0,8.88-2.84,29.09,10.84a100.26,100.26,0,0,1,53,0C198,88.3,206.9,91.14,206.9,91.14c5.76,14.56,2.14,25.32,1,28a40.87,40.87,0,0,1,10.89,28.39c0,40.62-24.74,49.56-48.29,52.18,3.79,3.28,7.17,9.71,7.17,19.58,0,14.15-.12,25.54-.12,29,0,2.82,1.9,6.11,7.26,5.07A105.76,105.76,0,0,0,151.36,47.28Z">
                </path>
                <path class="cls-12" d="M85.66,199.12c-.23.52-1.06.68-1.81.32s-1.2-1.06-.95-1.59,1.06-.69,1.82-.33,1.21,1.07.94,1.6Zm-1.3-1">
                </path>
                <path class="cls-12" d="M90,203.89c-.51.47-1.49.25-2.16-.49a1.61,1.61,0,0,1-.31-2.19c.52-.47,1.47-.25,2.17.49s.82,1.72.3,2.19Zm-1-1.08">
                </path>
                <path class="cls-12" d="M94.12,210c-.65.46-1.71,0-2.37-.91s-.64-2.07,0-2.52,1.7,0,2.36.89.65,2.08,0,2.54Zm0,0"></path>
                <path class="cls-12" d="M99.83,215.87c-.58.64-1.82.47-2.72-.41s-1.18-2.06-.6-2.7,1.83-.46,2.74.41,1.2,2.07.58,2.7Zm0,0">
                </path>
                <path class="cls-12" d="M107.71,219.29c-.26.82-1.45,1.2-2.64.85s-2-1.34-1.74-2.17,1.44-1.23,2.65-.85,2,1.32,1.73,2.17Zm0,0">
                </path>
                <path class="cls-12" d="M116.36,219.92c0,.87-1,1.59-2.24,1.61s-2.29-.68-2.3-1.54,1-1.59,2.26-1.61,2.28.67,2.28,1.54Zm0,0">
                </path>
                <path class="cls-12" d="M124.42,218.55c.15.85-.73,1.72-2,1.95s-2.37-.3-2.52-1.14.73-1.75,2-2,2.37.29,2.53,1.16Zm0,0"></path>
              </svg></a>
            <a class="footer-icon" id="footer_blog" href="https://ncbiinsights.ncbi.nlm.nih.gov/" aria-label="Blog">
              <svg id="Layer_1" data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 40 40"><defs><style>.cls-1{fill:#737373;}</style></defs><path class="cls-1" d="M14,30a4,4,0,1,1-4-4,4,4,0,0,1,4,4Zm11,3A19,19,0,0,0,7.05,15a1,1,0,0,0-1,1v3a1,1,0,0,0,.93,1A14,14,0,0,1,20,33.07,1,1,0,0,0,21,34h3a1,1,0,0,0,1-1Zm9,0A28,28,0,0,0,7,6,1,1,0,0,0,6,7v3a1,1,0,0,0,1,1A23,23,0,0,1,29,33a1,1,0,0,0,1,1h3A1,1,0,0,0,34,33Z"></path></svg>
            </a>
          </div>
        </div>
      </section>

      <section class="container-fluid bg-primary">
        <div class="container pt-5">
          <div class="row mt-3">
            <div class="col-lg-3 col-12">
              <p><a class="text-white" href="https://www.nlm.nih.gov/socialmedia/index.html">Connect with NLM</a></p>
              <ul class="list-inline social_media">
                <li class="list-inline-item"><a href="https://twitter.com/NLM_NIH" aria-label="Twitter" target="_blank" rel="noopener noreferrer"><svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <style type="text/css">
                        .st20 {
                          fill: #FFFFFF;
                        }

                        .st30 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }
                      </style>
                      <title>SM-Twitter</title>
                      <g>
                        <g>
                          <g>
                            <path class="st20" d="M192.9,88.1c-5,2.2-9.2,2.3-13.6,0.1c5.7-3.4,6-5.8,8.1-12.3c-5.4,3.2-11.4,5.5-17.6,6.7
                                                c-10.5-11.2-28.1-11.7-39.2-1.2c-7.2,6.8-10.2,16.9-8,26.5c-22.3-1.1-43.1-11.7-57.2-29C58,91.6,61.8,107.9,74,116
                                                c-4.4-0.1-8.7-1.3-12.6-3.4c0,0.1,0,0.2,0,0.4c0,13.2,9.3,24.6,22.3,27.2c-4.1,1.1-8.4,1.3-12.5,0.5c3.6,11.3,14,19,25.9,19.3
                                                c-11.6,9.1-26.4,13.2-41.1,11.5c12.7,8.1,27.4,12.5,42.5,12.5c51,0,78.9-42.2,78.9-78.9c0-1.2,0-2.4-0.1-3.6
                                                C182.7,97.4,189.2,93.7,192.9,88.1z"></path>
                          </g>
                        </g>
                        <circle class="st30" cx="124.4" cy="128.8" r="108.2"></circle>
                      </g>
                    </svg></a></li>
                <li class="list-inline-item"><a href="https://www.facebook.com/nationallibraryofmedicine" aria-label="Facebook" rel="noopener noreferrer" target="_blank">
                    <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <style type="text/css">
                        .st10 {
                          fill: #FFFFFF;
                        }

                        .st110 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }
                      </style>
                      <title>SM-Facebook</title>
                      <g>
                        <g>
                          <path class="st10" d="M159,99.1h-24V88.4c0-5,3.3-6.2,5.7-6.2h16.8V60l-24.4-0.1c-22.1,0-26.2,16.5-26.2,27.1v12.1H90v22.5h16.9
                                                      v67.5H135v-67.5h21.7L159,99.1z"></path>
                        </g>
                      </g>
                      <circle class="st110" cx="123.6" cy="123.2" r="108.2"></circle>
                    </svg>
                  </a></li>
                <li class="list-inline-item"><a href="https://www.youtube.com/user/NLMNIH" aria-label="Youtube" target="_blank" rel="noopener noreferrer"><svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <title>SM-Youtube</title>
                      <style type="text/css">
                        .st4 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }

                        .st5 {
                          fill: #FFFFFF;
                        }
                      </style>
                      <circle class="st4" cx="124.2" cy="123.4" r="108.2"></circle>
                      <g transform="translate(0,-952.36218)">
                        <path class="st5" d="M88.4,1037.4c-10.4,0-18.7,8.3-18.7,18.7v40.1c0,10.4,8.3,18.7,18.7,18.7h72.1c10.4,0,18.7-8.3,18.7-18.7
                                            v-40.1c0-10.4-8.3-18.7-18.7-18.7H88.4z M115.2,1058.8l29.4,17.4l-29.4,17.4V1058.8z"></path>
                      </g>
                    </svg></a></li>
              </ul>
            </div>
            <div class="col-lg-3 col-12">
              <p class="address_footer text-white">National Library of Medicine<br>
                <a href="https://www.google.com/maps/place/8600+Rockville+Pike,+Bethesda,+MD+20894/@38.9959508,-77.101021,17z/data=!3m1!4b1!4m5!3m4!1s0x89b7c95e25765ddb:0x19156f88b27635b8!8m2!3d38.9959508!4d-77.0988323" class="text-white" target="_blank" rel="noopener noreferrer">8600 Rockville Pike<br>
                  Bethesda, MD 20894</a></p>
            </div>
            <div class="col-lg-3 col-12 centered-lg">
              <p><a href="https://www.nlm.nih.gov/web_policies.html" class="text-white">Web Policies</a><br>
                <a href="https://www.nih.gov/institutes-nih/nih-office-director/office-communications-public-liaison/freedom-information-act-office" class="text-white">FOIA</a><br>
                <a href="https://www.hhs.gov/vulnerability-disclosure-policy/index.html" class="text-white" id="vdp">HHS Vulnerability Disclosure</a></p>
            </div>
            <div class="col-lg-3 col-12 centered-lg">
              <p><a class="supportLink text-white" href="https://support.nlm.nih.gov/">Help</a><br>
                <a href="https://www.nlm.nih.gov/accessibility.html" class="text-white">Accessibility</a><br>
                <a href="https://www.nlm.nih.gov/careers/careers.html" class="text-white">Careers</a></p>
            </div>
          </div>
          <div class="row">
            <div class="col-lg-12 centered-lg">
              <nav class="bottom-links">
                <ul class="mt-3">
                  <li>
                    <a class="text-white" href="//www.nlm.nih.gov/">NLM</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.nih.gov/">NIH</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.hhs.gov/">HHS</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.usa.gov/">USA.gov</a>
                  </li>
                </ul>
              </nav>
            </div>
          </div>
        </div>
      </section>
    </footer>
<script type="text/javascript" src="js/nwds.js"></script>
<script type="text/javascript" src="js/ncbipopup.js"></script>
<script type="text/javascript" src="js/headerNew.js"></script>
<script src="js/uswds.min.js"></script>
        <script type="text/javascript" src="/portal/portal3rc.fcgi/rlib/js/InstrumentOmnitureBaseJS/InstrumentNCBIBaseJS/InstrumentPageStarterJS.js"></script>
<!--
<script type="text/javascript" src="://www.ncbi.nlm.nih.gov/portal/portal3rc.fcgi/supportedbrowsers/js/nonportalbc_min.js"></script>
<script type="text/javascript">$("#browsers_ajax").browserCheck();</script>
-->
   </div><!--/#wrap-->
</body>

</html>"##;

    const EXAMPLE_STATUS_UNKNOWN: &'static str = r##"
<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en">
<head>
<meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
<meta name="jig" content="ncbitoggler"/>
<meta name="ncbitoggler" content="animation:'none'"/>
<title>NCBI Blast:</title>
<script type="text/javascript" src="/core/jig/1.15.2/js/jig.min.js             "></script>
<script type="text/javascript">    jQuery.getScript("/core/alerts/alerts.js", function() {
        galert(['div#header', 'body > *:nth-child(1)'])
    });</script>
<meta http-equiv="Pragma" content="no-cache">
<link rel="stylesheet" type="text/css" href="css/uswds.min.css" media="screen" />
<link rel="stylesheet"  type="text/css" href="https://www.ncbi.nlm.nih.gov/style-guide/static/nwds/css/nwds.css"/>
<link rel="stylesheet" href="css/headerNew.css?v=1"/>
<link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.5.0/css/all.css" crossorigin="anonymous"> <!-- Font Awesome icons -->
<link rel="stylesheet" type="text/css" href="css/footerNew.css?v=1" media="screen" />
<link rel="stylesheet" type="text/css" href="css/main.css" media="screen" />
<link rel="stylesheet" type="text/css" href="css/blastRes.css" media="screen" />
<link rel="stylesheet" type="text/css" href="css/print.css" media="print" />
<!--[if lte IE 6]>
<link rel="stylesheet" type="text/css" href="css/ie6_or_less.css" />
<![endif]-->
<script type="text/javascript" src="js/utils.js"></script>
<script type="text/javascript" src="js/results.js"></script>
</head>

<body id="type-a" class="noToggleCheck" >
<div id="wrap">
		<script>var useOfficialGovtHeader = true;</script>
<section class="usa-banner">
  <div class="usa-accordion">
    <header class="usa-banner-header">
      <div class="usa-grid usa-banner-inner">
        <img src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/favicons/favicon-57.png" alt="U.S. flag">
        <p>An official website of the United States government</p>
        <button class="usa-accordion-button usa-banner-button" aria-expanded="false" aria-controls="gov-banner-top">
          <span class="usa-banner-button-text">Here's how you know</span>
        </button>
      </div>
    </header>
    <div class="usa-banner-content usa-grid usa-accordion-content" id="gov-banner-top" aria-hidden="true">
      <div class="usa-banner-guidance-gov usa-width-one-half">
        <img class="usa-banner-icon usa-media_block-img" src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/icon-dot-gov.svg" alt="Dot gov">
        <div class="usa-media_block-body">
          <p>
            <strong>The .gov means it’s official.</strong>
            <br>
            Federal government websites often end in .gov or .mil. Before
            sharing sensitive information, make sure you’re on a federal
            government site.
          </p>
        </div>
      </div>
      <div class="usa-banner-guidance-ssl usa-width-one-half">
        <img class="usa-banner-icon usa-media_block-img" src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/icon-https.svg" alt="Https">
        <div class="usa-media_block-body">
          <p>
            <strong>The site is secure.</strong>
            <br>
            The <strong>https://</strong> ensures that you are connecting to the
            official website and that any information you provide is encrypted
            and transmitted securely.
          </p>
        </div>
      </div>
    </div>
  </div>
</section>
    	
<header class="ncbi-header" role="banner" data-section="Header">
<a class="usa-skipnav" href="#mainCont">Skip to main page content</a>
<div class="usa-grid">
    <div class="usa-width-one-whole">
        <div class="ncbi-header__logo">
                <a href="https://www.ncbi.nlm.nih.gov/" class="logo" aria-label="NCBI Logo" data-ga-action="click_image" data-ga-label="NIH NLM Logo">
                  <img src="https://www.ncbi.nlm.nih.gov/coreutils/nwds/img/logos/AgencyLogo.svg" alt="NIH NLM Logo">
                </a>
            </div>

        <div class="ncbi-header__account">
            <a id="account_login" href="https://www.ncbi.nlm.nih.gov/account/?back_url=https%3A%2F%2Fblast%2Encbi%2Enlm%2Enih%2Egov%2FBlast%2Ecgi%3FCMD%3DGet%26FORMAT%5FOBJECT%3DSearchInfo%26RID%3DNJVT5XRT013" class="usa-button header-button">Log in</a>
            <button id="account_info" class="header-button" aria-controls="account_popup">
                <span class="fa fa-user" aria-hidden="true"></span>
                <span class="username desktop-only" aria-hidden="true" id="uname_short"></span>
                <span class="sr-only">Show account info</span>
            </button>
        </div>

        <div class="ncbi-popup-anchor">
            <div class="ncbi-popup account-popup" id="account_popup" aria-hidden="true" role="dialog" aria-labelledby="account-popup-header">
                <div class="ncbi-popup-head">
                    <button class="ncbi-close-button"><span class="fa fa-window-close"></span><span class="usa-sr-only">Close</span></button>
                    <h4>Account</h4>
                </div>
                <div class="account-user-info">
                    Logged in as:<br>
                    <b><span class="username" id="uname_long">username</span></b>
                </div>
                <div class="account-links">
                    <ul class="usa-unstyled-list">
                        <li><a id="account_myncbi" href="https://www.ncbi.nlm.nih.gov/myncbi/">Dashboard</a> <span class="ncbi-text-small-light">(My NCBI)</span></li>
                        <li><a id="account_pubs" href="https://www.ncbi.nlm.nih.gov/myncbi/collections/bibliography/">Publications</a> <span class="ncbi-text-small-light">(My Bibliography)</span></li>
                        <li><a id="account_settings" href="https://www.ncbi.nlm.nih.gov/account/settings/">Account settings</a></li>
                        <li><a id="account_logout" href="https://www.ncbi.nlm.nih.gov/account/signout/?back_url=https%3A%2F%2Fblast%2Encbi%2Enlm%2Enih%2Egov%2FBlast%2Ecgi%3FCMD%3DGet%26FORMAT%5FOBJECT%3DSearchInfo%26RID%3DNJVT5XRT013">Log out</a></li>
                    </ul>
                </div>
            </div>
        </div>

    </div>
</div>
</header>
<div role="navigation" aria-label="access keys">
<a id="nws_header_accesskey_0" href="https://www.ncbi.nlm.nih.gov/guide/browsers/#ncbi_accesskeys" class="usa-sr-only" accesskey="0" tabindex="-1">Access keys</a>
<a id="nws_header_accesskey_1" href="https://www.ncbi.nlm.nih.gov" class="usa-sr-only" accesskey="1" tabindex="-1">NCBI Homepage</a>
<a id="nws_header_accesskey_2" href="/myncbi/" class="set-base-url usa-sr-only" accesskey="2" tabindex="-1">MyNCBI Homepage</a>
<a id="nws_header_accesskey_3" href="#maincontent" class="usa-sr-only" accesskey="3" tabindex="-1">Main Content</a>
<a id="nws_header_accesskey_4" href="#" class="usa-sr-only" accesskey="4" tabindex="-1">Main Navigation</a>
</div>
<nav class="ncbi-topnav" id="navcontent">
    <div class="usa-grid">
        <a class="ncbi-topnav-root" href="Blast.cgi">BLAST <sup>&reg;</sup></a> <span id="brc"><span class="brcrmbsign">&raquo;</span> <a href="Blast.cgi?PAGE=Nucleotides&amp;PROGRAM=blastn&amp;PAGE_TYPE=BlastSearch&amp;BLAST_SPEC=">blastn suite</a> <span class="brcrmbsign">&raquo;</span> RID-NJVT5XRT013</span>
        <ul class="rf ncbi-topnav-list" id="topnav-list">
            <li class="first "><a href="Blast.cgi?CMD=Web&amp;PAGE_TYPE=BlastHome" title="BLAST Home">Home</a></li>
            <li class="recent "><a href="Blast.cgi?CMD=GetSaved&amp;RECENT_RESULTS=on" title="Unexpired BLAST jobs">Recent Results</a></li>                
            <li class="saved "><a href="Blast.cgi?CMD=GetSaved" title="Saved sets of BLAST search parameters">Saved Strategies</a></li>
            <li  class= "last documentation "> <a href="../doc/blast-help/" title="BLAST documentation">Help</a></li>                            
        </ul>
    </div>
</nav>



        <div id="content-wrap">

                <div class="pageTitle">                            
                   Format Request Status                   
                </div>
		<div class="inlineDiv resHeader">				   
				   <a  id="frmPage"  class="WAITING" href="#" submitForm="reformat">[Formatting options] </a>                   
                </div>
                <h2 id="jtitle" >Job Title: </h2>
								
                <div id="content">                
                <!--<ul id="msg" class="msg"><li class=""><p class=""></p><p class=""></p><p class=""></p></ul> -->
                <ul id="msg" class="msg"><li class=""></li></ul>
                <p><!--
                QBlastInfoBegin
	                Status=UNKNOWN
                QBlastInfoEnd
                --></p> 
                               
                <SCRIPT LANGUAGE="JavaScript"><!--
                    var tm = "2000";
                    if (tm != "") {                    
                        setTimeout('document.forms[0].submit();',tm);
                    }
                //--></SCRIPT>                                
                <table id="statInfo" class="WAITING">
                <tr><td>Request ID</td><td> <b>NJVT5XRT013</b></td></tr>
                <tr class="odd"><td>Status</td><td>Searching</td></tr>
                <tr><td>Submitted at</td><td>Wed Oct 26 13:32:21 2022</td></tr>
                <tr class="odd"><td>Current time</td><td>Wed Oct 26 13:33:06 2022</td></tr>
                <tr><td>Time since submission</td><td>00:00:45</td></tr>
                </table>
                <p class="WAITING">This page will be automatically updated in <b>2</b> seconds</p>       
                <form action="Blast.cgi" enctype="application/x-www-form-urlencoded" method="POST" id="results">
                <input name="FORMAT_OBJECT" type="hidden" value="SearchInfo"><input name="RID" type="hidden" value="NJVT5XRT013"><input name="SEARCH_DB_STATUS" type="hidden" value="21"><input name="USER_TYPE" type="hidden" value="2"><input name="_PGR" type="hidden" value="0">                
                <input name="_PGR" type="hidden" value="0" >
                <input name="CMD" type="hidden" value="Get">
               
                </form>
                                
				</div><!-- /#content -->
				<form action="Blast.cgi" enctype="application/x-www-form-urlencoded"  method="post" name="reformat" id="reformat">				
				   <input name="QUERY_INFO" type="hidden" value="" />    				
				   <input name="ENTREZ_QUERY" type="hidden" value="" />
                   <input name="CDD_RID" type="hidden" value="" />
                   <input name="CDD_SEARCH_STATE" type="hidden" value="" />
                   <input name="RID" type="hidden" value="NJVT5XRT013" />
				   <input name="STEP_NUMBER" type="hidden" value="" />
				   <input name="CMD" type="hidden" value="Web"/>				
				   <input NAME="PAGE_TYPE" type="hidden"  value="BlastFormatting"/>
				
				   <!-- TO DO: test all of those changes -->				   	
				   <!-- Psi blast params  PSI_BLAST_PARAMS - commented- using forms[0] from fromatter> -->
				   <!-- Current Formatting options FORMATTING_OPTIONS- commented- using forms[0] from fromatter> -->
				   <!-- Current Search options CURR_SAVED_OPTIONS - commented- using forms[0] from fromatter> -->
                 </form>				
        </div><!-- /#content-wrap -->

         <footer>
      <section class="icon-section">
        <div id="icon-section-header" class="icon-section_header">Follow NCBI</div>
        <div class="grid-container container">
          <div class="icon-section_container">
            <a class="footer-icon" id="footer_twitter" href="https://twitter.com/ncbi" aria-label="Twitter"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <defs>
                  <style>
                    .cls-11 {
                      fill: #737373;
                    }
                  </style>
                </defs>
                <title>Twitter</title>
                <path class="cls-11" d="M250.11,105.48c-7,3.14-13,3.25-19.27.14,8.12-4.86,8.49-8.27,11.43-17.46a78.8,78.8,0,0,1-25,9.55,39.35,39.35,0,0,0-67,35.85,111.6,111.6,0,0,1-81-41.08A39.37,39.37,0,0,0,81.47,145a39.08,39.08,0,0,1-17.8-4.92c0,.17,0,.33,0,.5a39.32,39.32,0,0,0,31.53,38.54,39.26,39.26,0,0,1-17.75.68,39.37,39.37,0,0,0,36.72,27.3A79.07,79.07,0,0,1,56,223.34,111.31,111.31,0,0,0,116.22,241c72.3,0,111.83-59.9,111.83-111.84,0-1.71,0-3.4-.1-5.09C235.62,118.54,244.84,113.37,250.11,105.48Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_facebook" href="https://www.facebook.com/ncbi.nlm" aria-label="Facebook"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <title>Facebook</title>
                <path class="cls-11" d="M210.5,115.12H171.74V97.82c0-8.14,5.39-10,9.19-10h27.14V52l-39.32-.12c-35.66,0-42.42,26.68-42.42,43.77v19.48H99.09v36.32h27.24v109h45.41v-109h35Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_linkedin" href="https://www.linkedin.com/company/ncbinlm" aria-label="LinkedIn"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <title>LinkedIn</title>
                <path class="cls-11" d="M101.64,243.37H57.79v-114h43.85Zm-22-131.54h-.26c-13.25,0-21.82-10.36-21.82-21.76,0-11.65,8.84-21.15,22.33-21.15S101.7,78.72,102,90.38C102,101.77,93.4,111.83,79.63,111.83Zm100.93,52.61A17.54,17.54,0,0,0,163,182v61.39H119.18s.51-105.23,0-114H163v13a54.33,54.33,0,0,1,34.54-12.66c26,0,44.39,18.8,44.39,55.29v58.35H198.1V182A17.54,17.54,0,0,0,180.56,164.44Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_github" href="https://github.com/ncbi" aria-label="GitHub"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <defs>
                  <style>
                    .cls-11,
                    .cls-12 {
                      fill: #737373;
                    }

                    .cls-11 {
                      fill-rule: evenodd;
                    }
                  </style>
                </defs>
                <title>GitHub</title>
                <path class="cls-11" d="M151.36,47.28a105.76,105.76,0,0,0-33.43,206.1c5.28,1,7.22-2.3,7.22-5.09,0-2.52-.09-10.85-.14-19.69-29.42,6.4-35.63-12.48-35.63-12.48-4.81-12.22-11.74-15.47-11.74-15.47-9.59-6.56.73-6.43.73-6.43,10.61.75,16.21,10.9,16.21,10.9,9.43,16.17,24.73,11.49,30.77,8.79,1-6.83,3.69-11.5,6.71-14.14C108.57,197.1,83.88,188,83.88,147.51a40.92,40.92,0,0,1,10.9-28.39c-1.1-2.66-4.72-13.42,1-28,0,0,8.88-2.84,29.09,10.84a100.26,100.26,0,0,1,53,0C198,88.3,206.9,91.14,206.9,91.14c5.76,14.56,2.14,25.32,1,28a40.87,40.87,0,0,1,10.89,28.39c0,40.62-24.74,49.56-48.29,52.18,3.79,3.28,7.17,9.71,7.17,19.58,0,14.15-.12,25.54-.12,29,0,2.82,1.9,6.11,7.26,5.07A105.76,105.76,0,0,0,151.36,47.28Z">
                </path>
                <path class="cls-12" d="M85.66,199.12c-.23.52-1.06.68-1.81.32s-1.2-1.06-.95-1.59,1.06-.69,1.82-.33,1.21,1.07.94,1.6Zm-1.3-1">
                </path>
                <path class="cls-12" d="M90,203.89c-.51.47-1.49.25-2.16-.49a1.61,1.61,0,0,1-.31-2.19c.52-.47,1.47-.25,2.17.49s.82,1.72.3,2.19Zm-1-1.08">
                </path>
                <path class="cls-12" d="M94.12,210c-.65.46-1.71,0-2.37-.91s-.64-2.07,0-2.52,1.7,0,2.36.89.65,2.08,0,2.54Zm0,0"></path>
                <path class="cls-12" d="M99.83,215.87c-.58.64-1.82.47-2.72-.41s-1.18-2.06-.6-2.7,1.83-.46,2.74.41,1.2,2.07.58,2.7Zm0,0">
                </path>
                <path class="cls-12" d="M107.71,219.29c-.26.82-1.45,1.2-2.64.85s-2-1.34-1.74-2.17,1.44-1.23,2.65-.85,2,1.32,1.73,2.17Zm0,0">
                </path>
                <path class="cls-12" d="M116.36,219.92c0,.87-1,1.59-2.24,1.61s-2.29-.68-2.3-1.54,1-1.59,2.26-1.61,2.28.67,2.28,1.54Zm0,0">
                </path>
                <path class="cls-12" d="M124.42,218.55c.15.85-.73,1.72-2,1.95s-2.37-.3-2.52-1.14.73-1.75,2-2,2.37.29,2.53,1.16Zm0,0"></path>
              </svg></a>
            <a class="footer-icon" id="footer_blog" href="https://ncbiinsights.ncbi.nlm.nih.gov/" aria-label="Blog">
              <svg id="Layer_1" data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 40 40"><defs><style>.cls-1{fill:#737373;}</style></defs><path class="cls-1" d="M14,30a4,4,0,1,1-4-4,4,4,0,0,1,4,4Zm11,3A19,19,0,0,0,7.05,15a1,1,0,0,0-1,1v3a1,1,0,0,0,.93,1A14,14,0,0,1,20,33.07,1,1,0,0,0,21,34h3a1,1,0,0,0,1-1Zm9,0A28,28,0,0,0,7,6,1,1,0,0,0,6,7v3a1,1,0,0,0,1,1A23,23,0,0,1,29,33a1,1,0,0,0,1,1h3A1,1,0,0,0,34,33Z"></path></svg>
            </a>
          </div>
        </div>
      </section>

      <section class="container-fluid bg-primary">
        <div class="container pt-5">
          <div class="row mt-3">
            <div class="col-lg-3 col-12">
              <p><a class="text-white" href="https://www.nlm.nih.gov/socialmedia/index.html">Connect with NLM</a></p>
              <ul class="list-inline social_media">
                <li class="list-inline-item"><a href="https://twitter.com/NLM_NIH" aria-label="Twitter" target="_blank" rel="noopener noreferrer"><svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <style type="text/css">
                        .st20 {
                          fill: #FFFFFF;
                        }

                        .st30 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }
                      </style>
                      <title>SM-Twitter</title>
                      <g>
                        <g>
                          <g>
                            <path class="st20" d="M192.9,88.1c-5,2.2-9.2,2.3-13.6,0.1c5.7-3.4,6-5.8,8.1-12.3c-5.4,3.2-11.4,5.5-17.6,6.7
                                                c-10.5-11.2-28.1-11.7-39.2-1.2c-7.2,6.8-10.2,16.9-8,26.5c-22.3-1.1-43.1-11.7-57.2-29C58,91.6,61.8,107.9,74,116
                                                c-4.4-0.1-8.7-1.3-12.6-3.4c0,0.1,0,0.2,0,0.4c0,13.2,9.3,24.6,22.3,27.2c-4.1,1.1-8.4,1.3-12.5,0.5c3.6,11.3,14,19,25.9,19.3
                                                c-11.6,9.1-26.4,13.2-41.1,11.5c12.7,8.1,27.4,12.5,42.5,12.5c51,0,78.9-42.2,78.9-78.9c0-1.2,0-2.4-0.1-3.6
                                                C182.7,97.4,189.2,93.7,192.9,88.1z"></path>
                          </g>
                        </g>
                        <circle class="st30" cx="124.4" cy="128.8" r="108.2"></circle>
                      </g>
                    </svg></a></li>
                <li class="list-inline-item"><a href="https://www.facebook.com/nationallibraryofmedicine" aria-label="Facebook" rel="noopener noreferrer" target="_blank">
                    <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <style type="text/css">
                        .st10 {
                          fill: #FFFFFF;
                        }

                        .st110 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }
                      </style>
                      <title>SM-Facebook</title>
                      <g>
                        <g>
                          <path class="st10" d="M159,99.1h-24V88.4c0-5,3.3-6.2,5.7-6.2h16.8V60l-24.4-0.1c-22.1,0-26.2,16.5-26.2,27.1v12.1H90v22.5h16.9
                                                      v67.5H135v-67.5h21.7L159,99.1z"></path>
                        </g>
                      </g>
                      <circle class="st110" cx="123.6" cy="123.2" r="108.2"></circle>
                    </svg>
                  </a></li>
                <li class="list-inline-item"><a href="https://www.youtube.com/user/NLMNIH" aria-label="Youtube" target="_blank" rel="noopener noreferrer"><svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <title>SM-Youtube</title>
                      <style type="text/css">
                        .st4 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }

                        .st5 {
                          fill: #FFFFFF;
                        }
                      </style>
                      <circle class="st4" cx="124.2" cy="123.4" r="108.2"></circle>
                      <g transform="translate(0,-952.36218)">
                        <path class="st5" d="M88.4,1037.4c-10.4,0-18.7,8.3-18.7,18.7v40.1c0,10.4,8.3,18.7,18.7,18.7h72.1c10.4,0,18.7-8.3,18.7-18.7
                                            v-40.1c0-10.4-8.3-18.7-18.7-18.7H88.4z M115.2,1058.8l29.4,17.4l-29.4,17.4V1058.8z"></path>
                      </g>
                    </svg></a></li>
              </ul>
            </div>
            <div class="col-lg-3 col-12">
              <p class="address_footer text-white">National Library of Medicine<br>
                <a href="https://www.google.com/maps/place/8600+Rockville+Pike,+Bethesda,+MD+20894/@38.9959508,-77.101021,17z/data=!3m1!4b1!4m5!3m4!1s0x89b7c95e25765ddb:0x19156f88b27635b8!8m2!3d38.9959508!4d-77.0988323" class="text-white" target="_blank" rel="noopener noreferrer">8600 Rockville Pike<br>
                  Bethesda, MD 20894</a></p>
            </div>
            <div class="col-lg-3 col-12 centered-lg">
              <p><a href="https://www.nlm.nih.gov/web_policies.html" class="text-white">Web Policies</a><br>
                <a href="https://www.nih.gov/institutes-nih/nih-office-director/office-communications-public-liaison/freedom-information-act-office" class="text-white">FOIA</a><br>
                <a href="https://www.hhs.gov/vulnerability-disclosure-policy/index.html" class="text-white" id="vdp">HHS Vulnerability Disclosure</a></p>
            </div>
            <div class="col-lg-3 col-12 centered-lg">
              <p><a class="supportLink text-white" href="https://support.nlm.nih.gov/">Help</a><br>
                <a href="https://www.nlm.nih.gov/accessibility.html" class="text-white">Accessibility</a><br>
                <a href="https://www.nlm.nih.gov/careers/careers.html" class="text-white">Careers</a></p>
            </div>
          </div>
          <div class="row">
            <div class="col-lg-12 centered-lg">
              <nav class="bottom-links">
                <ul class="mt-3">
                  <li>
                    <a class="text-white" href="//www.nlm.nih.gov/">NLM</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.nih.gov/">NIH</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.hhs.gov/">HHS</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.usa.gov/">USA.gov</a>
                  </li>
                </ul>
              </nav>
            </div>
          </div>
        </div>
      </section>
    </footer>
<script type="text/javascript" src="js/nwds.js"></script>
<script type="text/javascript" src="js/ncbipopup.js"></script>
<script type="text/javascript" src="js/headerNew.js"></script>
<script src="js/uswds.min.js"></script>
        <script type="text/javascript" src="/portal/portal3rc.fcgi/rlib/js/InstrumentOmnitureBaseJS/InstrumentNCBIBaseJS/InstrumentPageStarterJS.js"></script>
<!--
<script type="text/javascript" src="://www.ncbi.nlm.nih.gov/portal/portal3rc.fcgi/supportedbrowsers/js/nonportalbc_min.js"></script>
<script type="text/javascript">$("#browsers_ajax").browserCheck();</script>
-->
   </div><!--/#wrap-->
</body>

</html>"##;

    const EXAMPLE_QUERY: &'static str = r##"<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Transitional//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd">
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en" lang="en">
<head>
<meta http-equiv="Content-Type" content="text/html; charset=UTF-8" />
<meta name="jig" content="ncbitoggler ncbiautocomplete"/>
<meta name="ncbi_app" content="static" />
<meta name="ncbi_pdid" content="blastformatreq" />
<meta name="ncbi_stat" content="false" />
<meta name="ncbi_sessionid" content="55BA0BD535971F61_0000SID" />
<meta name="ncbi_phid" content="55BA0BD535971F610000000000000001" />
<title>NCBI Blast</title>
<meta http-equiv="Pragma" content="no-cache">
<link rel="stylesheet" type="text/css" href="css/uswds.min.css" media="screen" />
<link rel="stylesheet"  type="text/css" href="https://www.ncbi.nlm.nih.gov/style-guide/static/nwds/css/nwds.css"/>
<link rel="stylesheet" href="css/headerNew.css?v=1"/>
<link rel="stylesheet" href="https://use.fontawesome.com/releases/v5.5.0/css/all.css" crossorigin="anonymous"> <!-- Font Awesome icons -->
<link rel="stylesheet" type="text/css" href="css/footerNew.css?v=1" media="screen" />
<link rel="stylesheet" type="text/css" href="css/main.css" media="screen" />
<link rel="stylesheet" type="text/css" href="css/common.css" media="screen" />
<link rel="stylesheet" type="text/css" href="css/blastReq.css" media="screen" />
<!--[if IE]>
<link rel="stylesheet" type="text/css" href="css/blastReqIE.css" media="screen" />
<![endif]-->
<link rel="stylesheet" type="text/css" href="css/print.css" media="print" />


<!--[if lte IE 6]>
<link rel="stylesheet" type="text/css" href="css/ie6_or_less.css" />
<![endif]-->
<script type="text/javascript" src="/core/jig/1.15.2/js/jig.min.js             "></script>   
<script type="text/javascript" src="js/utils.js"></script>
<script type="text/javascript" src="js/blast.js"></script>
<script type="text/javascript" src="js/format.js"></script>

</head>

<body id="type-a">

<div id="wrap">
		<script>var useOfficialGovtHeader = true;</script>
<section class="usa-banner">
  <div class="usa-accordion">
    <header class="usa-banner-header">
      <div class="usa-grid usa-banner-inner">
        <img src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/favicons/favicon-57.png" alt="U.S. flag">
        <p>An official website of the United States government</p>
        <button class="usa-accordion-button usa-banner-button" aria-expanded="false" aria-controls="gov-banner-top">
          <span class="usa-banner-button-text">Here's how you know</span>
        </button>
      </div>
    </header>
    <div class="usa-banner-content usa-grid usa-accordion-content" id="gov-banner-top" aria-hidden="true">
      <div class="usa-banner-guidance-gov usa-width-one-half">
        <img class="usa-banner-icon usa-media_block-img" src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/icon-dot-gov.svg" alt="Dot gov">
        <div class="usa-media_block-body">
          <p>
            <strong>The .gov means it’s official.</strong>
            <br>
            Federal government websites often end in .gov or .mil. Before
            sharing sensitive information, make sure you’re on a federal
            government site.
          </p>
        </div>
      </div>
      <div class="usa-banner-guidance-ssl usa-width-one-half">
        <img class="usa-banner-icon usa-media_block-img" src="https://www.ncbi.nlm.nih.gov/coreutils/uswds/img/icon-https.svg" alt="Https">
        <div class="usa-media_block-body">
          <p>
            <strong>The site is secure.</strong>
            <br>
            The <strong>https://</strong> ensures that you are connecting to the
            official website and that any information you provide is encrypted
            and transmitted securely.
          </p>
        </div>
      </div>
    </div>
  </div>
</section>
    	
<header class="ncbi-header" role="banner" data-section="Header">
<a class="usa-skipnav" href="#mainCont">Skip to main page content</a>
<div class="usa-grid">
    <div class="usa-width-one-whole">
        <div class="ncbi-header__logo">
                <a href="https://www.ncbi.nlm.nih.gov/" class="logo" aria-label="NCBI Logo" data-ga-action="click_image" data-ga-label="NIH NLM Logo">
                  <img src="https://www.ncbi.nlm.nih.gov/coreutils/nwds/img/logos/AgencyLogo.svg" alt="NIH NLM Logo">
                </a>
            </div>

        <div class="ncbi-header__account">
            <a id="account_login" href="https://www.ncbi.nlm.nih.gov/account/?back_url=https%3A%2F%2Fblast%2Encbi%2Enlm%2Enih%2Egov%2FBlast%2Ecgi%3FCMD%3DPut%26DATABASE%3Dnt%26ENTREZ%5FQUERY%3D%26FULL%5FDBNAME%3Dnt%26JOB%5FTITLE%3DProtein%2BSequence%2B%26MYNCBI%5FUSER%3D11650217443%26MYNCBI%5FUSER%3D11650217443%26ORG%5FDBS%3Ddbvers5%26QUERY%5FINFO%3DProtein%2BSequence%2B%26QUERY%5FLENGTH%3D19%26RID%3DNJWFSE65016%26RTOE%3D46%26USER%5FTYPE%3D2%26USER%5FTYPE%3D2" class="usa-button header-button">Log in</a>
            <button id="account_info" class="header-button" aria-controls="account_popup">
                <span class="fa fa-user" aria-hidden="true"></span>
                <span class="username desktop-only" aria-hidden="true" id="uname_short"></span>
                <span class="sr-only">Show account info</span>
            </button>
        </div>

        <div class="ncbi-popup-anchor">
            <div class="ncbi-popup account-popup" id="account_popup" aria-hidden="true" role="dialog" aria-labelledby="account-popup-header">
                <div class="ncbi-popup-head">
                    <button class="ncbi-close-button"><span class="fa fa-window-close"></span><span class="usa-sr-only">Close</span></button>
                    <h4>Account</h4>
                </div>
                <div class="account-user-info">
                    Logged in as:<br>
                    <b><span class="username" id="uname_long">username</span></b>
                </div>
                <div class="account-links">
                    <ul class="usa-unstyled-list">
                        <li><a id="account_myncbi" href="https://www.ncbi.nlm.nih.gov/myncbi/">Dashboard</a> <span class="ncbi-text-small-light">(My NCBI)</span></li>
                        <li><a id="account_pubs" href="https://www.ncbi.nlm.nih.gov/myncbi/collections/bibliography/">Publications</a> <span class="ncbi-text-small-light">(My Bibliography)</span></li>
                        <li><a id="account_settings" href="https://www.ncbi.nlm.nih.gov/account/settings/">Account settings</a></li>
                        <li><a id="account_logout" href="https://www.ncbi.nlm.nih.gov/account/signout/?back_url=https%3A%2F%2Fblast%2Encbi%2Enlm%2Enih%2Egov%2FBlast%2Ecgi%3FCMD%3DPut%26DATABASE%3Dnt%26ENTREZ%5FQUERY%3D%26FULL%5FDBNAME%3Dnt%26JOB%5FTITLE%3DProtein%2BSequence%2B%26MYNCBI%5FUSER%3D11650217443%26MYNCBI%5FUSER%3D11650217443%26ORG%5FDBS%3Ddbvers5%26QUERY%5FINFO%3DProtein%2BSequence%2B%26QUERY%5FLENGTH%3D19%26RID%3DNJWFSE65016%26RTOE%3D46%26USER%5FTYPE%3D2%26USER%5FTYPE%3D2">Log out</a></li>
                    </ul>
                </div>
            </div>
        </div>

    </div>
</div>
</header>
<div role="navigation" aria-label="access keys">
<a id="nws_header_accesskey_0" href="https://www.ncbi.nlm.nih.gov/guide/browsers/#ncbi_accesskeys" class="usa-sr-only" accesskey="0" tabindex="-1">Access keys</a>
<a id="nws_header_accesskey_1" href="https://www.ncbi.nlm.nih.gov" class="usa-sr-only" accesskey="1" tabindex="-1">NCBI Homepage</a>
<a id="nws_header_accesskey_2" href="/myncbi/" class="set-base-url usa-sr-only" accesskey="2" tabindex="-1">MyNCBI Homepage</a>
<a id="nws_header_accesskey_3" href="#maincontent" class="usa-sr-only" accesskey="3" tabindex="-1">Main Content</a>
<a id="nws_header_accesskey_4" href="#" class="usa-sr-only" accesskey="4" tabindex="-1">Main Navigation</a>
</div>
<nav class="ncbi-topnav" id="navcontent">
    <div class="usa-grid">
        <a class="ncbi-topnav-root" href="Blast.cgi">BLAST <sup>&reg;</sup></a> <span id="brc"></span>
        <ul class="rf ncbi-topnav-list" id="topnav-list">
            <li class="first "><a href="Blast.cgi?CMD=Web&amp;PAGE_TYPE=BlastHome" title="BLAST Home">Home</a></li>
            <li class="recent "><a href="Blast.cgi?CMD=GetSaved&amp;RECENT_RESULTS=on" title="Unexpired BLAST jobs">Recent Results</a></li>                
            <li class="saved "><a href="Blast.cgi?CMD=GetSaved" title="Saved sets of BLAST search parameters">Saved Strategies</a></li>
            <li  class= "last documentation "> <a href="../doc/blast-help/" title="BLAST documentation">Help</a></li>                            
        </ul>
    </div>
</nav>


        <div id="content-wrap">

		<div class="pageTitle">                            
                   Format Request
                   <span id="frmRequestPrTr"></span>                   
                </div>
				<!-- Do errors this way -->				
				<!--<ul class="msg"><li class=""><p></p></li></ul>-->
				<ul id="msgR" class="msg"><li class=""></li></ul>
                <div id="content">
				<form action="Blast.cgi" enctype="application/x-www-form-urlencoded" method="post" name="FormatForm" id="FormatForm">				

<script language="JavaScript">

 <!--

//document.images['BlastHeaderGif'].src = 'html/head_formating.gif';

// -->

</script>



<!--
                <p class='info'>
<strong>Job submitted.</strong>
We estimate that results will be ready in 16 seconds or less.

</p>
-->

<div class="fbtn">
<!--
<a href="javascript:document.forms[0].submit();">
<img align="middle" alt="Format button" border="0" src="FormatPage_files/format_but.gif">
</a>
-->
</div>

<dl class="summary  query title db">
<dd>
</dd>

<!-- <span class=" query title db">-->
<dt class="hidden query">Query</dt><dd class="hidden query">Protein Sequence</dd>
<dt class="hidden db">Database</dt><dd class="hidden db">nt</dd>
<dt class="hidden title">Job title</dt><dd class="hidden title">Protein Sequence</dd>
<dt class="hidden entrez">Entrez Query</dt><dd class="hidden entrez"><span class="note entrez">Note: Your search is limited to records matching this Entrez query</span></dd>
<!-- </span> -->
<dt><label for="rid">Request ID</label></dt><dd><input name="RID" size="50" type="text" value="NJWFSE65016" id="rid" />
<input type="submit" value="View report" name="ViewReport" class="button" />
<!-- <img border="0" id="viewRpButton" src="images/veiwRpButton.jpg" class="viewReport"  alt="View report"  mouseovImg="images/veiwRpButtonOver.jpg" mouseoutImg="images/veiwRpButton.jpg" mousedownImg="images/veiwRpButtonDown.jpg" mouseupImg="images/veiwRpButtonOver.jpg"  />-->
<input type="checkbox" name="NEWWINRES"  form="FormatForm" winType="const" id="nw" class="newwin"  />
<label for="nw">Show results in a new window</label>
</dd>
<dt>Format<br/>
<!--<a class='help' href="#">[Help]</a></dt> -->

<dd>
<table id="filterResults" class="options  ">

<tr class="paramSet xgl">
<td class="hd"><label for="FORMAT_OBJECT">Show</label></td>
<td>
<div class="fi">
<select id="FORMAT_OBJECT" class="reset" name="FORMAT_OBJECT" defVal="Alignment">
<option value="Alignment" >Alignment</option>
<option value="PSSM_Scoremat" >PssmWithParameters</option>
<option value="Bioseq"  >Bioseq</option>
</select>
<label for="FORMAT_TYPE">as</label>
<select name="FORMAT_TYPE" id="FORMAT_TYPE" class="reset" defVal="HTML">
<option value="HTML"  >HTML</option>
<option value="Text"  >Plain text</option>
<option value="ASN.1"  >ASN.1</option>
<option value="XML"  >XML</option>
<option value="XML2"  >XML2</option>
<option value="JSON2"  >JSON2</option>
<option value="XML2_S"  >XML2_S</option>
<option value="JSON2_S"  >JSON2_S</option>
<option value="SAM_SQ"  >SAM_SQ</option>
<option value="SAM"  >SAM</option>
</select>
<input name="PSSM_FORMAT_TYPE" value="Text" size="3" id="pssmFormat" type="text" class="hidden dispType" />
<input name="BIOSEQ_FORMAT_TYPE" value="ASN.1" size="3" id="bioseqFormat" type="text" class="hidden dispType" />
<input name="PSSM_SC_FORMAT_TYPE" value="ASN.1" size="3" id="pssmScFormat" type="text" class="hidden dispType" />
<a class="resetAll" id="resetAll" >Reset form to defaults</a>
<a class="helplink  ui-ncbitoggler" data-jig="ncbitoggler" title="Alignments object formatting help" id="formatHelp" href="#"><i class="fas fa-question-circle"></i><span class="usa-sr-only">Help</span></a>
<div class="ui-helper-reset" aria-live="assertive" >
<p class="helpbox ui-ncbitoggler-slave" id="hlp1">
These options control formatting of alignments in results pages. The
default is HTML, but other formats (including plain text) are available.
PSSM and PssmWithParameters are representations of Position Specific Scoring Matrices and are only available for PSI-BLAST. 
The Advanced view option allows the database descriptions to be sorted by various indices in a table.
</p>
</div><!-- ARIA -->
</div>
</td>
</tr>

<tr class="odd paramSet">
<td class="hd"><label for="ALIGNMENT_VIEW">Alignment View</label></td>
<td>
<div class="fi">
<select name="ALIGNMENT_VIEW" id="ALIGNMENT_VIEW" defVal="Pairwise" class="reset">
<option value="Pairwise"  >Pairwise</option>
<option value="PairwiseWithIdentities"  >Pairwise with dots for identities</option>
<option value="QueryAnchored"  >Query-anchored with dots for identities</option>
<option value="QueryAnchoredNoIdentities"  >Query-anchored with letters for identities</option>
<option value="FlatQueryAnchored"  >Flat query-anchored with dots for identities</option>
<option value="FlatQueryAnchoredNoIdentities"  >Flat query-anchored with letters for identities</option>
<option value="Tabular"  >Hit Table</option>
</select>

<a class="helplink  ui-ncbitoggler" data-jig="ncbitoggler" title="Alignments view options help" id="alnViewHelp" href="#"><i class="fas fa-question-circle"></i><span class="usa-sr-only">Help</span></a>
<div class="ui-helper-reset" aria-live="assertive" >
<p class="helpbox ui-ncbitoggler-slave" id="hlp2">
Choose how to view alignments.
The default "pairwise" view shows how each subject sequence aligns
individually to the query sequence. The "query-anchored" view shows how
all subject sequences align to the query sequence. For each view type,
you can choose to show "identities" (matching residues) as letters or
dots.
<a href="Blast.cgi?CMD=Web&amp;PAGE_TYPE=BlastDocs&amp;DOC_TYPE=BlastHelp#alignment_view" target="helpWin" title="Additional alignments view options help">more...</a>
</p>
</div><!-- ARIA -->
</div>
</td>
</tr>

<tr class="paramSet">
<td class="hd"><label>Display</label></td>
<td class="cb">
<div class="fi">
<input name="SHOW_OVERVIEW" id="SHOW_OVERVIEW" type="checkbox" class="cb reset" defVal="checked" checked="checked" />
<label class="rb" for="SHOW_OVERVIEW">Graphical Overview</label>

<span id="shl" >
<input name="SHOW_LINKOUT" id="SHOW_LINKOUT" type="checkbox" class="cb reset" defVal="checked" checked="checked" />
<label class="rb" for="SHOW_LINKOUT">Linkout</label>
</span>
<span id="gts" >
<input name="GET_SEQUENCE" id="GET_SEQUENCE" type="checkbox" class="cb reset" defVal="checked" checked="checked" />
<label class="rb" for="GET_SEQUENCE">Sequence Retrieval</label>
</span>

<input name="NCBI_GI" id="NCBI_GI" type="checkbox" class="cb reset " defVal="unchecked"  />
<label class="rb " for="NCBI_GI">NCBI-gi</label>
<span id="scf" >
<input name="SHOW_CDS_FEATURE" id="SHOW_CDS_FEATURE" type="checkbox" class="cb reset blastn" defVal="unchecked"  />
<label for="SHOW_CDS_FEATURE" class="blastn">CDS feature</label>
</span>
<a class="helplink  ui-ncbitoggler" data-jig="ncbitoggler" title="Alignments display options help" id="displayHelp" href="#"><i class="fas fa-question-circle"></i><span class="usa-sr-only">Help</span></a>
<div class="ui-helper-reset" aria-live="assertive" >
<ul class="helpbox ui-ncbitoggler-slave" id="hlp3">
<li>Graphical Overview: Graphical Overview: Show graph of similar sequence regions aligned to  query.
<a href="Blast.cgi?CMD=Web&amp;PAGE_TYPE=BlastDocs&amp;DOC_TYPE=BlastHelp#show_overview" target="helpWin" title="Graphical Overview help">more...</a>
</li>
<li>NCBI-gi: Show NCBI gi identifiers.
</li>
<li>CDS feature: Show annotated coding region and translation.
<a href="Blast.cgi?CMD=Web&amp;PAGE_TYPE=BlastDocs&amp;DOC_TYPE=BlastHelp#show_cds_feature" title="CDS feature help" target="helpWin" >more...</a>
</li></ul>
</div><!-- ARIA -->
</div>
</td>
</tr>


<tr class="paramSet odd xgl">
<td class="hd"><label>Masking</label></td>
<td>
<div class="fi">
<label for="MASK_CHAR"> Character: </label>
<select name="MASK_CHAR" id="MASK_CHAR"  class="reset" defVal="2">
<option value="0"  >X for protein, n for nucleotide</option>
<option value="2" selected="selected" >Lower Case</option>
</select>
<label for="MASK_COLOR"> Color:</label>
<select name="MASK_COLOR" id="MASK_COLOR" class="reset" defVal="1">
<option value="0"  >Black
</option>

<option value="1" selected="selected" >Grey
</option>

<option value="2"  >Red
</option>

</select>
<a class="helplink  ui-ncbitoggler" data-jig="ncbitoggler" title="Alignments masking help" id="maskingHelp" href="#"><i class="fas fa-question-circle"></i><span class="usa-sr-only">Help</span></a>
<div class="ui-helper-reset" aria-live="assertive" >
<ul class="helpbox ui-ncbitoggler-slave" id="hlp4">
<li>Masking Character: Display masked (filtered) sequence regions as lower-case or as specific letters (N for nucleotide, P for protein).
</li>
<li>Masking Color: Display masked sequence regions in the given color.</li>
</ul>
</div><!-- ARIA -->
</div>
</td>
</tr>


<tr id="lr" class="paramSet xgl">
<td class="hd"><label>Limit results</label></td>
<td>
<div class="fi">
<label for="FRM_DESCRIPTIONS">Descriptions:</label>
<select name="DESCRIPTIONS" id="FRM_DESCRIPTIONS" class="reset" defVal="100">
<option value="0"      >0</option>
<option value="10"     >10</option>
<option value="50"     >50</option>
<option value="100"   selected="selected" >100</option>
<option value="250"    >250</option>
<option value="500"    >500</option>
<option value="1000"   >1000</option>
<option value="5000"   >5000</option>
<option value="10000"  >10000</option>
<option value="20000"  >20000</option>
</select>

<label for="FRM_NUM_OVERVIEW">Graphical overview:</label>
<select name="NUM_OVERVIEW" id="FRM_NUM_OVERVIEW" class="reset" defVal="100">
<option value="0"     >0</option>
<option value="10"    >10</option>
<option value="50"    >50</option>
<option value="100"  selected="selected" >100</option>
<option value="250"   >250</option>
<option value="500"  >500</option>
<option value="1000"  >1000</option>
</select>
<span id="frmAln">
<label for="FRM_ALIGNMENTS">Alignments:</label>
<select name="ALIGNMENTS" id="FRM_ALIGNMENTS" class="reset" defVal="100">
<option value="0"      >0</option>
<option value="10"     >10</option>
<option value="50"     >50</option>
<option value="100"   selected="selected" >100</option>
<option value="250"    >250</option>
<option value="500"    >500</option>
<option value="1000"   >1000</option>
<option value="5000"   >5000</option>
<option value="10000"  >10000</option>
<option value="20000"  >20000</option>
</select>
</span>
<label for="FRM_LINE_LENGTH">Line length:</label>
<select name="LINE_LENGTH" id="FRM_LINE_LENGTH" class="reset" defVal="60">
<option value="60"     >60</option>
<option value="90"     >90</option>
<option value="120"     >120</option>
<option value="150"     >150</option>
</select>
<a class="helplink  ui-ncbitoggler" data-jig="ncbitoggler" title="Limit number of descriptions/alignments help" id="numHelp" href="#"><i class="fas fa-question-circle"></i><span class="usa-sr-only">Help</span></a>
<div class="ui-helper-reset" aria-live="assertive" >
<ul class="helpbox ui-ncbitoggler-slave" id="hlp5">
<li>Descriptions: Show short descriptions for up to the given number of  sequences.</li> 
<li>Alignments:  Show alignments for up to the given number of sequences, in order of statistical significance.</li>
<li>Line lenghth:  Number of letters to show on one line in an alignment.</li>
</ul>
</div><!-- ARIA -->
</div>
</td>
</tr>

<tr class="paramSet odd xgl ">
<td class="hd"></td>
<td>
<div class="">
<label for="qorganism">Organism</label>
<span class="instr">Type common name, binomial, taxid, or group name. Only 20 top taxa will be shown.</span><br/>
<input name="FORMAT_ORGANISM" size="55"  type="text" id="qorganism" value="" data-jigconfig="dictionary:'taxids_sg',isCrossDomain:false" autocomplete="off" data-jig="ncbiautocomplete" class="reset">
<input type="hidden" value = "1" name="FORMAT_NUM_ORG" id="numOrg" />
<input type="checkbox" name="FORMAT_ORG_EXCLUDE"  class="oExclR cb" id="orgExcl"/>        
<label for="orgExcl" class="right">exclude</label>
<a href="#" title="Add organism" id="addOrg"><img border="0" src="css/images/addOrg.jpg" id="addOrgIm"   alt="Add organism"  mouseovImg="css/images/addOrgOver.jpg" mouseoutImg="css/images/addOrg.jpg" mousedownImg="css/images/addOrgDown.jpg" mouseupImg="css/images/addOrgOver.jpg"  /></a>
<div id="orgs">

</div>
<div class="fi">
<a class="helplink  ui-ncbitoggler" data-jig="ncbitoggler" title="Limit results by organism help" id="organismHelp" href="#"><i class="fas fa-question-circle"></i><span class="usa-sr-only">Help</span></a>
<div class="ui-helper-reset" aria-live="assertive" >
<p class="helpbox ui-ncbitoggler-slave" id="hlp6">
Show only sequences from the given organism.
</p>
</div><!-- ARIA -->
</div>
</div>
</td>
</tr>

<tr class="paramSet xgl ">
<td class="hd"></td>
<td>
<div class="fi">
<label for="FORMAT_EQ_TEXT">Entrez query:</label>
<input name="FORMAT_EQ_TEXT" id="FORMAT_EQ_TEXT" size="60" type="text" value="" class="reset" />
<a class="helplink  ui-ncbitoggler" data-jig="ncbitoggler" title="Limit results by Entrez query help" id="entrezHelp" href="#"><i class="fas fa-question-circle"></i><span class="usa-sr-only">Help</span></a>
<div class="ui-helper-reset" aria-live="assertive" >
<p class="helpbox ui-ncbitoggler-slave" id="hlp7">
Show only those sequences that match the given Entrez query.
<a href="Blast.cgi?CMD=Web&amp;PAGE_TYPE=BlastDocs&amp;DOC_TYPE=BlastHelp#limit_result" target="helpWin" title="Additional limit results by Entrez query help"  target="helpWin">more...</a>
</p>
</div><!-- ARIA -->
</div>
</td>
</tr>

  
<tr class="paramSet odd xgl">
<td class="hd"></td>
<td>
<div class="fi">
<label for="EXPECT_LOW">Expect Min:</label> <input name="EXPECT_LOW" id="EXPECT_LOW" size="10" type="text" value="" class="reset"/>
<label for="EXPECT_HIGH">Expect Max:</label> <input name="EXPECT_HIGH" id="EXPECT_HIGH" size="10" type="text" value="" class="reset" />
<a class="helplink  ui-ncbitoggler" data-jig="ncbitoggler" title="Limit results by expect value range help" id="expectHelp" href="#"><i class="fas fa-question-circle"></i><span class="usa-sr-only">Help</span></a>
<div class="ui-helper-reset" aria-live="assertive" >
<p class="helpbox ui-ncbitoggler-slave" id="hlp8">
Show only sequences with expect values in the given range.
<a href="Blast.cgi?CMD=Web&amp;PAGE_TYPE=BlastDocs&amp;DOC_TYPE=BlastHelp#expect_range" target="helpWin" title="Additional limit results by expect value range help">more...</a>
</p>
</div><!-- ARIA -->
</div>
</td>
</tr>
<tr class="paramSet xgl">
<td class="hd"></td>
<td>
 <div class="fi">
<label for="PERC_IDENT_LOW">Percent Identity Min:</label> <input name="PERC_IDENT_LOW" id="PERC_IDENT_LOW" size="10" type="text" value="" class="reset"/>
<label for="PERC_IDENT_HIGH">Percent Identity Max:</label> <input name="PERC_IDENT_HIGH" id="PERC_IDENT_HIGH" size="10" type="text" value="" class="reset" />
<a class="helplink  ui-ncbitoggler" data-jig="ncbitoggler" title="Limit results by percent identity range help" id="percIdentHelp" href="#"><i class="fas fa-question-circle"></i><span class="usa-sr-only">Help</span></a>
<div class="ui-helper-reset" aria-live="assertive" >
<p class="helpbox ui-ncbitoggler-slave" id="hlp10">
 Show only sequences with percent identity values in the given range.  
</p>
</div><!-- ARIA -->
</div>
</td>
</tr>      
<tr class="psiBlast odd paramSet xgl ">
<td class="hd"><label>Format for</label></td>
<td>
<div class="fi">
<input name="RUN_PSIBLAST_FORM" id="RUN_PSIBLAST" type="checkbox" class="cb psiBlast hidden"  />
<label for="I_THRESH">PSI-BLAST with inclusion threshold:</label>
<input name="I_THRESH" id="I_THRESH" size="10" type="text" value="" defVal="0.005" />
<a class="helplink  ui-ncbitoggler" data-jig="ncbitoggler" title="PSI BLAST formatting help" id="psiHelp" href="#"><i class="fas fa-question-circle"></i><span class="usa-sr-only">Help</span></a>
<div class="ui-helper-reset" aria-live="assertive" >
<ul class="helpbox ui-ncbitoggler-slave" id="hlp9">
<li>Format for PSI-BLAST: The Position-Specific Iterated BLAST (PSI-BLAST) program performs iterative searches with a protein query, 
in which sequences found in one round of search are used to build a custom score model for the next round.
<a href="Blast.cgi?CMD=Web&amp;PAGE_TYPE=BlastDocs&amp;DOC_TYPE=BlastHelp#psiblast" target="helpWin" title="Additional PSI BLAST formatting help">more...</a>
</li>  
<li>Inclusion Threshold: This sets the statistical significance threshold for including a sequence in the model used 
by PSI-BLAST to create the PSSM on the next iteration.</li> 
</ul>
</div><!-- ARIA -->
</div>
</td>
</tr>
</table>
</dd>
</dl>

<input name="RID" value="NJWFSE65016" type="hidden" />
<input name="CDD_RID" value="" type="hidden" />
<input name="CDD_SEARCH_STATE" type="hidden" value="" />

<input name="STEP_NUMBER" value="" id="stepNumber" type="hidden" />
<input name="CMD" value="Get" type="hidden" />
<input name="FORMAT_EQ_OP" value="AND" type="hidden" />
<input name="RESULTS_PAGE_TARGET" type="hidden" id="resPageTarget" value="Blast_Results_for_1296528764" />
<input name="QUERY_INFO" type="hidden" value="Protein Sequence" />                   		
<input name="ENTREZ_QUERY" type="hidden" value="" />
<input name="QUERY_INDEX" type="hidden" value="0"/>
<input name="NUM_QUERIES" type="hidden" value="1"/>
<input name="CONFIG_DESCR" type="hidden" value="ClustMemNbr,ClustComn,Ds,Sc,Ms,Ts,Cov,Eval,Idnt,AccLen,Acc" />




<!-- Those params are set in the template (blastn.dat, blastp.dat etc. -->
<input name="BLAST_PROGRAMS" type="hidden" value=""/>
<input name="PAGE" type="hidden" value=""/>
<input name="PROGRAM" type="hidden" value=""/>
<input name="MEGABLAST" type="hidden" value="" />
<input name="RUN_PSIBLAST" type="hidden" value="" />
<input name="BLAST_SPEC" id="blastSpec" type="hidden" value=""/>


<input name="QUERY" type="hidden" value=""/>
<input name="JOB_TITLE" type="hidden" value="Protein Sequence"/>
<input name="QUERY_TO" type="hidden" value=""/>
<input name="QUERY_FROM" type="hidden" value=""/>
<input name="SUBJECTS_FROM" type="hidden" value=""/>
<input name="SUBJECTS_TO" type="hidden" value=""/>
<input name="EQ_TEXT" type="hidden" value=""/>
<input name="ORGN" type="hidden" value=""/>
<input name="EQ_MENU" type="hidden" value=""/>
<input name="ORG_EXCLUDE" type="hidden" value=""/>
<input name="PHI_PATTERN" type="hidden" value=""/>
<input name="EXPECT" type="hidden" value=""/>									
<input name="DATABASE" type="hidden" value="nt"/>
<input name="DB_GROUP" type="hidden" value=""/>
<input name="SUBGROUP_NAME" type="hidden" value=""/>

<input name="GENETIC_CODE" type="hidden" value=""/>
<input name="WORD_SIZE" type="hidden" value=""/>
<input name="MATCH_SCORES" type="hidden" value=""/>			
<input name="MATRIX_NAME" type="hidden" value=""/>				
<input name="GAPCOSTS" type="hidden" value=""/>
<input name="MAX_NUM_SEQ" id="maxNumSeq" type="hidden" value=""/>					
<input name="COMPOSITION_BASED_STATISTICS" type="hidden" value=""/>			
<input name="NEWWIN" type="hidden" value=""/>
<input name="SHORT_QUERY_ADJUST" type="hidden" value=""/>
<input name="FILTER" type="hidden" value=""/>
<input name="REPEATS" type="hidden" value=""/>
<input name="ID_FOR_PSSM" type="hidden" value=""/>
<input name="EXCLUDE_MODELS" type="hidden" value=""/>
<input name="EXCLUDE_SEQ_UNCULT" type="hidden" value=""/>
<input name="WP_PROTEINS" type="hidden" value=""/>
<input name="SEQ_FROM_TYPE" type="hidden" value=""/>
<input name="ENTREZ_QUERY" type="hidden" value=""/>
<input name="ENTREZ_QUERY_PRESET" type="hidden" value=""/>
<input name="ENTREZ_QUERY_PRESET_EXCL" type="hidden" value=""/>
<input name="NUM_ORG" type="hidden" value = "1" />

<!-- PSSM -->
<input name="LCASE_MASK" type="hidden" value=""/>
<input name="TEMPLATE_TYPE" type="hidden" value=""/>
<input name="TEMPLATE_LENGTH" type="hidden" value=""/>
<input name="I_THRESH" type="hidden" value=""/>
<input name="PSI_PSEUDOCOUNT" type="hidden" value=""/>
<input name="DI_THRESH" type="hidden" id="diThresh" value=""/>
<input name="HSP_RANGE_MAX" type="hidden" value=""/>



<input name="ADJUSTED_FOR_SHORT_QUERY" type="hidden" value=""/>
<input name="MIXED_QUERIES" type="hidden" value=""/>
<input name="MIXED_DATABASE" id="mixedDb" type="hidden" value=""/>
<input name="BUILD_NAME"  type="hidden" value=""/>
<input name="ORG_DBS"  type="hidden" value="dbvers5"/>
<input name="WWW_BLAST_TYPE" type="hidden" value=""/>

<!--QBlastInfoBegin
    RID = NJWFSE65016
    RTOE = 46
QBlastInfoEnd
-->
</form>		                              		
				
				</div><!-- /#content -->

        </div><!-- /#content-wrap -->

		 <footer>
      <section class="icon-section">
        <div id="icon-section-header" class="icon-section_header">Follow NCBI</div>
        <div class="grid-container container">
          <div class="icon-section_container">
            <a class="footer-icon" id="footer_twitter" href="https://twitter.com/ncbi" aria-label="Twitter"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <defs>
                  <style>
                    .cls-11 {
                      fill: #737373;
                    }
                  </style>
                </defs>
                <title>Twitter</title>
                <path class="cls-11" d="M250.11,105.48c-7,3.14-13,3.25-19.27.14,8.12-4.86,8.49-8.27,11.43-17.46a78.8,78.8,0,0,1-25,9.55,39.35,39.35,0,0,0-67,35.85,111.6,111.6,0,0,1-81-41.08A39.37,39.37,0,0,0,81.47,145a39.08,39.08,0,0,1-17.8-4.92c0,.17,0,.33,0,.5a39.32,39.32,0,0,0,31.53,38.54,39.26,39.26,0,0,1-17.75.68,39.37,39.37,0,0,0,36.72,27.3A79.07,79.07,0,0,1,56,223.34,111.31,111.31,0,0,0,116.22,241c72.3,0,111.83-59.9,111.83-111.84,0-1.71,0-3.4-.1-5.09C235.62,118.54,244.84,113.37,250.11,105.48Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_facebook" href="https://www.facebook.com/ncbi.nlm" aria-label="Facebook"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <title>Facebook</title>
                <path class="cls-11" d="M210.5,115.12H171.74V97.82c0-8.14,5.39-10,9.19-10h27.14V52l-39.32-.12c-35.66,0-42.42,26.68-42.42,43.77v19.48H99.09v36.32h27.24v109h45.41v-109h35Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_linkedin" href="https://www.linkedin.com/company/ncbinlm" aria-label="LinkedIn"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <title>LinkedIn</title>
                <path class="cls-11" d="M101.64,243.37H57.79v-114h43.85Zm-22-131.54h-.26c-13.25,0-21.82-10.36-21.82-21.76,0-11.65,8.84-21.15,22.33-21.15S101.7,78.72,102,90.38C102,101.77,93.4,111.83,79.63,111.83Zm100.93,52.61A17.54,17.54,0,0,0,163,182v61.39H119.18s.51-105.23,0-114H163v13a54.33,54.33,0,0,1,34.54-12.66c26,0,44.39,18.8,44.39,55.29v58.35H198.1V182A17.54,17.54,0,0,0,180.56,164.44Z">
                </path>
              </svg></a>
            <a class="footer-icon" id="footer_github" href="https://github.com/ncbi" aria-label="GitHub"><svg data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 300">
                <defs>
                  <style>
                    .cls-11,
                    .cls-12 {
                      fill: #737373;
                    }

                    .cls-11 {
                      fill-rule: evenodd;
                    }
                  </style>
                </defs>
                <title>GitHub</title>
                <path class="cls-11" d="M151.36,47.28a105.76,105.76,0,0,0-33.43,206.1c5.28,1,7.22-2.3,7.22-5.09,0-2.52-.09-10.85-.14-19.69-29.42,6.4-35.63-12.48-35.63-12.48-4.81-12.22-11.74-15.47-11.74-15.47-9.59-6.56.73-6.43.73-6.43,10.61.75,16.21,10.9,16.21,10.9,9.43,16.17,24.73,11.49,30.77,8.79,1-6.83,3.69-11.5,6.71-14.14C108.57,197.1,83.88,188,83.88,147.51a40.92,40.92,0,0,1,10.9-28.39c-1.1-2.66-4.72-13.42,1-28,0,0,8.88-2.84,29.09,10.84a100.26,100.26,0,0,1,53,0C198,88.3,206.9,91.14,206.9,91.14c5.76,14.56,2.14,25.32,1,28a40.87,40.87,0,0,1,10.89,28.39c0,40.62-24.74,49.56-48.29,52.18,3.79,3.28,7.17,9.71,7.17,19.58,0,14.15-.12,25.54-.12,29,0,2.82,1.9,6.11,7.26,5.07A105.76,105.76,0,0,0,151.36,47.28Z">
                </path>
                <path class="cls-12" d="M85.66,199.12c-.23.52-1.06.68-1.81.32s-1.2-1.06-.95-1.59,1.06-.69,1.82-.33,1.21,1.07.94,1.6Zm-1.3-1">
                </path>
                <path class="cls-12" d="M90,203.89c-.51.47-1.49.25-2.16-.49a1.61,1.61,0,0,1-.31-2.19c.52-.47,1.47-.25,2.17.49s.82,1.72.3,2.19Zm-1-1.08">
                </path>
                <path class="cls-12" d="M94.12,210c-.65.46-1.71,0-2.37-.91s-.64-2.07,0-2.52,1.7,0,2.36.89.65,2.08,0,2.54Zm0,0"></path>
                <path class="cls-12" d="M99.83,215.87c-.58.64-1.82.47-2.72-.41s-1.18-2.06-.6-2.7,1.83-.46,2.74.41,1.2,2.07.58,2.7Zm0,0">
                </path>
                <path class="cls-12" d="M107.71,219.29c-.26.82-1.45,1.2-2.64.85s-2-1.34-1.74-2.17,1.44-1.23,2.65-.85,2,1.32,1.73,2.17Zm0,0">
                </path>
                <path class="cls-12" d="M116.36,219.92c0,.87-1,1.59-2.24,1.61s-2.29-.68-2.3-1.54,1-1.59,2.26-1.61,2.28.67,2.28,1.54Zm0,0">
                </path>
                <path class="cls-12" d="M124.42,218.55c.15.85-.73,1.72-2,1.95s-2.37-.3-2.52-1.14.73-1.75,2-2,2.37.29,2.53,1.16Zm0,0"></path>
              </svg></a>
            <a class="footer-icon" id="footer_blog" href="https://ncbiinsights.ncbi.nlm.nih.gov/" aria-label="Blog">
              <svg id="Layer_1" data-name="Layer 1" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 40 40"><defs><style>.cls-1{fill:#737373;}</style></defs><path class="cls-1" d="M14,30a4,4,0,1,1-4-4,4,4,0,0,1,4,4Zm11,3A19,19,0,0,0,7.05,15a1,1,0,0,0-1,1v3a1,1,0,0,0,.93,1A14,14,0,0,1,20,33.07,1,1,0,0,0,21,34h3a1,1,0,0,0,1-1Zm9,0A28,28,0,0,0,7,6,1,1,0,0,0,6,7v3a1,1,0,0,0,1,1A23,23,0,0,1,29,33a1,1,0,0,0,1,1h3A1,1,0,0,0,34,33Z"></path></svg>
            </a>
          </div>
        </div>
      </section>

      <section class="container-fluid bg-primary">
        <div class="container pt-5">
          <div class="row mt-3">
            <div class="col-lg-3 col-12">
              <p><a class="text-white" href="https://www.nlm.nih.gov/socialmedia/index.html">Connect with NLM</a></p>
              <ul class="list-inline social_media">
                <li class="list-inline-item"><a href="https://twitter.com/NLM_NIH" aria-label="Twitter" target="_blank" rel="noopener noreferrer"><svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <style type="text/css">
                        .st20 {
                          fill: #FFFFFF;
                        }

                        .st30 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }
                      </style>
                      <title>SM-Twitter</title>
                      <g>
                        <g>
                          <g>
                            <path class="st20" d="M192.9,88.1c-5,2.2-9.2,2.3-13.6,0.1c5.7-3.4,6-5.8,8.1-12.3c-5.4,3.2-11.4,5.5-17.6,6.7
                                                c-10.5-11.2-28.1-11.7-39.2-1.2c-7.2,6.8-10.2,16.9-8,26.5c-22.3-1.1-43.1-11.7-57.2-29C58,91.6,61.8,107.9,74,116
                                                c-4.4-0.1-8.7-1.3-12.6-3.4c0,0.1,0,0.2,0,0.4c0,13.2,9.3,24.6,22.3,27.2c-4.1,1.1-8.4,1.3-12.5,0.5c3.6,11.3,14,19,25.9,19.3
                                                c-11.6,9.1-26.4,13.2-41.1,11.5c12.7,8.1,27.4,12.5,42.5,12.5c51,0,78.9-42.2,78.9-78.9c0-1.2,0-2.4-0.1-3.6
                                                C182.7,97.4,189.2,93.7,192.9,88.1z"></path>
                          </g>
                        </g>
                        <circle class="st30" cx="124.4" cy="128.8" r="108.2"></circle>
                      </g>
                    </svg></a></li>
                <li class="list-inline-item"><a href="https://www.facebook.com/nationallibraryofmedicine" aria-label="Facebook" rel="noopener noreferrer" target="_blank">
                    <svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <style type="text/css">
                        .st10 {
                          fill: #FFFFFF;
                        }

                        .st110 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }
                      </style>
                      <title>SM-Facebook</title>
                      <g>
                        <g>
                          <path class="st10" d="M159,99.1h-24V88.4c0-5,3.3-6.2,5.7-6.2h16.8V60l-24.4-0.1c-22.1,0-26.2,16.5-26.2,27.1v12.1H90v22.5h16.9
                                                      v67.5H135v-67.5h21.7L159,99.1z"></path>
                        </g>
                      </g>
                      <circle class="st110" cx="123.6" cy="123.2" r="108.2"></circle>
                    </svg>
                  </a></li>
                <li class="list-inline-item"><a href="https://www.youtube.com/user/NLMNIH" aria-label="Youtube" target="_blank" rel="noopener noreferrer"><svg version="1.1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" x="0px" y="0px" viewBox="0 0 249 249" style="enable-background:new 0 0 249 249;" xml:space="preserve">
                      <title>SM-Youtube</title>
                      <style type="text/css">
                        .st4 {
                          fill: none;
                          stroke: #FFFFFF;
                          stroke-width: 8;
                          stroke-miterlimit: 10;
                        }

                        .st5 {
                          fill: #FFFFFF;
                        }
                      </style>
                      <circle class="st4" cx="124.2" cy="123.4" r="108.2"></circle>
                      <g transform="translate(0,-952.36218)">
                        <path class="st5" d="M88.4,1037.4c-10.4,0-18.7,8.3-18.7,18.7v40.1c0,10.4,8.3,18.7,18.7,18.7h72.1c10.4,0,18.7-8.3,18.7-18.7
                                            v-40.1c0-10.4-8.3-18.7-18.7-18.7H88.4z M115.2,1058.8l29.4,17.4l-29.4,17.4V1058.8z"></path>
                      </g>
                    </svg></a></li>
              </ul>
            </div>
            <div class="col-lg-3 col-12">
              <p class="address_footer text-white">National Library of Medicine<br>
                <a href="https://www.google.com/maps/place/8600+Rockville+Pike,+Bethesda,+MD+20894/@38.9959508,-77.101021,17z/data=!3m1!4b1!4m5!3m4!1s0x89b7c95e25765ddb:0x19156f88b27635b8!8m2!3d38.9959508!4d-77.0988323" class="text-white" target="_blank" rel="noopener noreferrer">8600 Rockville Pike<br>
                  Bethesda, MD 20894</a></p>
            </div>
            <div class="col-lg-3 col-12 centered-lg">
              <p><a href="https://www.nlm.nih.gov/web_policies.html" class="text-white">Web Policies</a><br>
                <a href="https://www.nih.gov/institutes-nih/nih-office-director/office-communications-public-liaison/freedom-information-act-office" class="text-white">FOIA</a><br>
                <a href="https://www.hhs.gov/vulnerability-disclosure-policy/index.html" class="text-white" id="vdp">HHS Vulnerability Disclosure</a></p>
            </div>
            <div class="col-lg-3 col-12 centered-lg">
              <p><a class="supportLink text-white" href="https://support.nlm.nih.gov/">Help</a><br>
                <a href="https://www.nlm.nih.gov/accessibility.html" class="text-white">Accessibility</a><br>
                <a href="https://www.nlm.nih.gov/careers/careers.html" class="text-white">Careers</a></p>
            </div>
          </div>
          <div class="row">
            <div class="col-lg-12 centered-lg">
              <nav class="bottom-links">
                <ul class="mt-3">
                  <li>
                    <a class="text-white" href="//www.nlm.nih.gov/">NLM</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.nih.gov/">NIH</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.hhs.gov/">HHS</a>
                  </li>
                  <li>
                    <a class="text-white" href="https://www.usa.gov/">USA.gov</a>
                  </li>
                </ul>
              </nav>
            </div>
          </div>
        </div>
      </section>
    </footer>
<script type="text/javascript" src="js/nwds.js"></script>
<script type="text/javascript" src="js/ncbipopup.js"></script>
<script type="text/javascript" src="js/headerNew.js"></script>
<script src="js/uswds.min.js"></script>
   </div><!--/#wrap-->

<script type="text/javascript" src="/portal/portal3rc.fcgi/rlib/js/InstrumentOmnitureBaseJS/InstrumentNCBIBaseJS/InstrumentPageStarterJS.js"></script>
<!--
<script type="text/javascript" src="://www.ncbi.nlm.nih.gov/portal/portal3rc.fcgi/supportedbrowsers/js/nonportalbc_min.js"></script>
<script type="text/javascript">$("#browsers_ajax").browserCheck();</script>
-->
</body>

</html>"##;

    #[test]
    fn test_parse_rid() {
        let rid = parse_rid(EXAMPLE_QUERY).unwrap();
        assert_eq!(rid, "NJWFSE65016".to_string())
    }

    #[test]
    fn test_parse_rid_missing() {
        let rid = parse_rid("no rid here");
        assert!(rid.is_err())
    }

    #[test]
    fn test_parse_rtoe() {
        let rtoe = parse_rtoe(EXAMPLE_QUERY).unwrap();
        assert_eq!(rtoe, 46);
    }

    #[test]
    fn test_parse_rtoe_missing() {
        let rtoe = parse_rtoe("no rtoe here");
        assert!(rtoe.is_err());
    }

    #[test]
    fn test_parse_status_waiting() {
        let status = parse_status(EXAMPLE_STATUS_WAITING).unwrap();
        assert_eq!(status, BlastStatus::Waiting);
    }

    #[test]
    fn test_parse_status_unknown() {
        let status = parse_status(EXAMPLE_STATUS_UNKNOWN).unwrap();
        assert_eq!(status, BlastStatus::Unknown);
    }

    #[test]
    fn test_parse_status_ready() {
        let status = parse_status(EXAMPLE_STATUS_READY).unwrap();
        assert_eq!(status, BlastStatus::Ready);
    }

    #[test]
    fn test_parse_status_missing() {
        let status = parse_status("no status here");
        assert!(status.is_err());
    }
}
