package dssinterface

// TimeRequest A json response from the server
type TimeRequest struct {
}

// TimeResponse A json response from the server
type TimeResponse struct {
	Ok     bool       `json:"ok"`
	Result TimeResult `json:"result"`
}

// TimeResult A json response from the server
type TimeResult struct {
	Time     int64  `json:"time"`
	Offset   int64  `json:"offset"`
	Daylight bool   `json:"daylight"`
	Timezone string `json:"timezone"`
}

// RequestApplicationTokenRequest A json response from the server
type RequestApplicationTokenRequest struct {
	ApplicationName string `url:"applicationName"`
}

// RequestApplicationTokenResponse A json response from the server
type RequestApplicationTokenResponse struct {
	Ok     bool                          `json:"ok"`
	Result RequestApplicationTokenResult `json:"result"`
}

// RequestApplicationTokenResult A json response from the server
type RequestApplicationTokenResult struct {
	ApplicationToken string `json:"applicationToken"`
}
