package cz.cvut.swa.bazaar.productservice.data

import org.springframework.data.annotation.Id
import org.springframework.data.mongodb.repository.MongoRepository
import java.time.LocalDateTime

data class Product(

        @Id val id: String,
        val title: String,
        val description: String,
        val postedDatetime: LocalDateTime = LocalDateTime.now()

)

interface ProductRepository : MongoRepository<Product, String>