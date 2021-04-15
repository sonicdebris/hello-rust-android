package rsonnicdebris.usty.droid

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import android.widget.TextView

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        System.loadLibrary("rustydroid")
        findViewById<TextView>(R.id.text).setText(hello("from rust"))
    }

    external fun hello(to: String): String
}